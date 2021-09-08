# Overview

A rustlet is a Rust closure that is used to extend the capabilities of servers that host applications accessed by means of the HTTP request-response programming model.

The rustlet [macro library](https://bitcoinmw.github.io/rustlet/librustlet/index.html) provides interfaces for writing rustlets. Rustlets are implemented via the [rustlet! macro](https://bitcoinmw.github.io/rustlet/librustlet/macro.rustlet.html). The other macros in the library provide all the functionalities required to respond to an HTTP Get or Post request.

# Execution of Rustlets

Rustlets are executed in the [nioruntime](https://github.com/bitcoinmw/nioruntime). This allows for performant execution using epoll on linux, kqueues on bsd variants including macos, and wepoll on windows.

# RSPs

RSPs are rust server pages. A RSP page is a text document that contains two types of text: static data, which can be expressed in any text-based format (but most commonly HTML), and rustlet tags which execute a specified rustlet as part of the page that is loading. A sample RSP may look like this:

```
<html>
    <head>
        <title>Sample RSP</title>
    </head>
    <body>
        <@=header>
        static content
        <@=middlecontent>
        static content
        <@=footer>
    </body>
</html>
```

In this example '<@=header>', '<@=middlecontent>', and '<@=footer>' are each rustlets that share the same parameters as the RSP page when executed. RSPs can be placed anywhere in the HTTP server's webroot and the rustlet container will interpret them to their dynamic form. RSP files must end with the .rsp extension so that the rustlet container knows to execute them as RSPs.

# Logging

The rustlet container comes with a logging library. The full documentation of the logging library can be [found here](https://bitcoinmw.github.io/rustlet/nioruntime_log/). This logging library uses the same syntax of the standard logging library for rust. See the example for info [here](https://bitcoinmw.github.io/rustlet/nioruntime_log/macro.info.html). Log level is set per file as seen in the previous example. The rustlet container itself uses this logging library for three log files. Each log file has a configurable location, max_size, and max_age. Further details about each of these log files is below.

## Main log

The mainlog prints out general messages. It mostly deals with log rotation messages and also prints out unexpected errors and information that that user should know about. On startup, all parameters are logged to the mainlog which is initially configured to print to standard out as well as it's file location. After startup, the mainlog is switched to only log to its file. A sample output of the mainlog looks like this:

```
[2021-09-07 20:10:09]: Rustlet Httpd 0.0.1
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-07 20:10:09]: root_dir:             '/Users/Shared/.niohttpd'
[2021-09-07 20:10:09]: webroot:              '/Users/Shared/.niohttpd/www'
[2021-09-07 20:10:09]: bind_address:         0.0.0.0:8080
[2021-09-07 20:10:09]: thread_pool_size:     8
[2021-09-07 20:10:09]: request_log_location: '/Users/Shared/.niohttpd/logs/request.log'
[2021-09-07 20:10:09]: request_log_max_size: 10.49 MB
[2021-09-07 20:10:09]: request_log_max_age:  60 Minutes
[2021-09-07 20:10:09]: mainlog_location:     '/Users/Shared/.niohttpd/logs/mainlog.log'
[2021-09-07 20:10:09]: mainlog_max_size:     10.49 MB
[2021-09-07 20:10:09]: mainlog_max_age:      360 Minutes
[2021-09-07 20:10:09]: stats_log_location:   '/Users/Shared/.niohttpd/logs/stats.log'
[2021-09-07 20:10:09]: stats_log_max_size:   10.49 MB
[2021-09-07 20:10:09]: stats_log_max_age:    360 Minutes
[2021-09-07 20:10:09]: stats_frequency:      10 Seconds
[2021-09-07 20:10:09]: max_log_queue:        100,000
[2021-09-07 20:10:09]: tls_cert_file:        None
[2021-09-07 20:10:09]: tls_private_key_file: None
[2021-09-07 20:10:09]: WARNING! flag set:    'debug'
[2021-09-07 20:10:09]: WARNING! flag set:    'delete_request_rotation'
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-07 20:10:09]: Server Started
```

## Request log

The request log is used to track information about individual requests. Each request is logged to the request log, but the actual logging takes place in another thread. If the server is overloaded, request logging can be dropped. The configured max_log_queue is the number of pending requests that are stored before request data is dropped and not logged to the request log. By default, this number is set to 100,000. The logging thread executes every 100 ms. Under normal circumstances, requests are not dropped with the default configurations. The data that is logged in the request log is configurable. See the confirguration section for more details on how to configure the request log.

A sample output of the mainlog looks like this:

```
|method|uri|query|User-Agent|Referer|ProcTime
[2021-09-07 20:12:36]: |GET|/||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.271693ms
[2021-09-07 20:12:36]: |GET|/lettherebebitcom.png||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/|2.871201ms
[2021-09-07 20:12:36]: |GET|/favicon-32x32.png||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/|0.280963ms
[2021-09-07 20:12:40]: |GET|/asdf||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.230612ms
[2021-09-07 20:12:40]: |GET|/DERP.jpeg||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/asdf|0.603154ms
[2021-09-07 20:12:42]: |GET|/printheaders||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.407074ms
[2021-09-07 20:12:43]: |GET|/favicon.ico||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36|http://localhost:8080/printheaders|0.334554ms
[2021-09-07 20:12:48]: |GET|/get_session||Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.124589ms
[2021-09-07 20:12:50]: |GET|/set_session|abc=109|Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36||0.119635ms
```

## Statistical log

The statistical log, shows statistics about the internals of the server. It can be used to see if there are slow pages and other information about the state of the server. The statistical log, logs data periodically. Two types of lines are logged.
* 1.) Data about the entire history of the server's current run.
* 2.) Data about the last n seconds of the server's current run, where n is configurable. The default value is every 5 seconds.

The data logged in this file are:
* 1.) REQUESTS - the number of requests (over the last n seconds or the entire history)
* 2.) CONNS - The current number of connections into the server.
* 3.) CONNECTS - The number of connections made to the server (over the last n seconds or the entire history).
* 4.) QPS - Queries per second. The number of queries made to the server per second (over the last n seconds or the entire history).
* 5.) IDLE_DISC - The number of connections which were disconnected after being idle for a period of time (the default is 2 minutes).
* 6.) RTIMEOUT - The number of connections which were disconnected after not making any requests at all for a period of time (the default is 30 seconds).
* 7.) AVG_LAT - The average latency per request (this includes the portion of the request that is either executing the rustlet or reading the file to return to the user for static HTTP requests.
* 8.) MAX_LAT - The maximum latency for a request.

A sample of the statistical log looks like this:

```
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      0 Days 00:01:00:        6,802,182          87          68,087   113369.70           0           0   0.0071864   24.933976
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-05 20:13:34]:        1,136,972          87          11,387   113697.20           0           0   0.0069253   13.433473
[2021-09-05 20:13:44]:        1,161,684          99          11,613   116168.40           0           0   0.0071729   23.659854
[2021-09-05 20:13:54]:        1,125,295          34          11,200   112529.50           0           0   0.0072360   14.536791
[2021-09-05 20:14:04]:        1,147,914          31          11,500   114791.40           0           0   0.0070697   13.358339
[2021-09-05 20:14:14]:        1,147,520          43          11,465   114752.00           0           0   0.0073007   13.523405
[2021-09-05 20:14:24]:        1,124,275          39          11,235   112427.50           0           0   0.0073382   15.702629
-------------------------------------------------------------------------------------------------------------------------------
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      0 Days 00:02:00:       13,649,810           8         136,500   113748.42           0           0   0.0072068   24.933976
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-05 20:14:34]:        1,140,940           8          11,400   114094.00           0           0   0.0072487   13.600388
[2021-09-05 20:14:44]:        1,143,399          80          11,480   114339.90           0           0   0.0069738   14.146019
[2021-09-05 20:14:54]:        1,146,106          37          11,420   114610.60           0           0   0.0072077   15.134817
[2021-09-05 20:15:04]:        1,142,332          41          11,442   114233.20           0           0   0.0071162   14.670403
[2021-09-05 20:15:14]:        1,126,276          43          11,258   112627.60           0           0   0.0072569   13.363555
[2021-09-05 20:15:24]:        1,143,816          34          11,443   114381.60           0           0   0.0072189   13.484573
-------------------------------------------------------------------------------------------------------------------------------
Statistical Log V_1.0:         REQUESTS       CONNS        CONNECTS         QPS   IDLE_DISC    RTIMEOUT     AVG_LAT     MAX_LAT
-------------------------------------------------------------------------------------------------------------------------------
      0 Days 00:03:00:       20,460,296          43         204,643   113668.31           0           0   0.0072020   24.933976
-------------------------------------------------------------------------------------------------------------------------------
[2021-09-05 20:15:34]:        1,108,557          43          11,100   110855.70           0           0   0.0073872   12.859597
[2021-09-05 20:15:44]:        1,149,210          19          11,457   114921.00           0           0   0.0072592   18.166237
[2021-09-05 20:15:54]:        1,135,703          63          11,395   113570.30           0           0   0.0070510   11.664569
[2021-09-05 20:16:04]:        1,133,285          75          11,305   113328.50           0           0   0.0072662   15.691763
[2021-09-05 20:16:14]:        1,129,587          81          11,300   112958.70           0           0   0.0072674   13.482526
[2021-09-05 20:16:24]:        1,156,421          67          11,574   115642.10           0           0   0.0070367   14.203071
```

# Configuration

The rustlet container is configured via the [rustlet-init](https://bitcoinmw.github.io/rustlet/librustlet/macro.rustlet_init.html) macro. All configuration structs implement the Default trait so the defaults can be used. Also, all of the fields are fully documented in the documentation linked to above.

# Samples

The Rustlet [macro library](https://bitcoinmw.github.io/rustlet/librustlet/index.html)  documentation provides numerous working examples. Also, the [rustlet-simple](https://github.com/bitcoinmw/rustlet-simple) project shows how to write and deploy a hello world rustlet in 3 easy steps. More examples to come...

