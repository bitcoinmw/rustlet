# Overview

A rustlet is a Rust closure that is used to extend the capabilities of servers that host applications accessed by means of the HTTP request-response programming model.

The rustlet [macro library](https://bitcoinmw.github.io/rustlet/librustlet/index.html) provides interfaces for writing rustlets. Rustlets are implemented via the [rustlet! macro](https://bitcoinmw.github.io/rustlet/librustlet/macro.rustlet.html). The other macros in the library provide all the functionalities required to respond to an HTTP Get or Post request.

# Execution of Rustlets

Rustlets are executed in the [nioruntime](https://github.com/bitcoinmw/nioruntime). This allows for performant execution using epoll on linux, kqueues on bsd variants, and wepoll on windows.

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

In this example '<@=header>', '<@=middlecontent>', and '<@=footer>' are each rustlets that share the same parameters as the RSP page when executed. RSPs can be placed anywhere in the HTTP server's webroot and the rustlet container will interpret them to their dynamic form.

# Samples

The Rustlet [macro library](https://bitcoinmw.github.io/rustlet/librustlet/index.html)  documentation provides numerous working examples. Also, the [rustlet-simple](https://github.com/bitcoinmw/rustlet-simple) project shows how to write and deploy a hello world rustlet in 3 easy steps. More example to come...

