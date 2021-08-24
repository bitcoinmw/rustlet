initSidebarItems({"macro":[["debug","Log at the ‘debug’ (1) log level. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. Also see [`trace`], [`info`], [`warn`], [`error`], or [`fatal`]."],["debug_no_ts","Just like [`debug`], but with no timestamp."],["do_log","Generally, this macro should not be used directly. It is used by the other macros. See [`log`] or [`info`] instead."],["error","Log at the ‘error’ (4) log level. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. Also see [`trace`], [`debug`], [`info`], [`warn`], or [`fatal`]."],["error_no_ts","Just like [`error`], but with no timestamp."],["fatal","Log at the ‘fatal’ (5) log level. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. Also see [`trace`] [`debug`], [`info`], [`warn`], or [`error`]."],["fatal_no_ts","Just like [`fatal`], but with no timestamp."],["info","Log at the ‘info’ (2) log level. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. Also see [`trace`], [`debug`], [`warn`], [`error`], or [`fatal`]."],["info_no_ts","Just like [`info`], but with no timestamp."],["log","The main logging macro. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. The first parameter is the log level. To avoid specifying level, see [`trace`], [`debug`], [`info`], [`warn`], [`error`], or [`fatal`]."],["log_config","This macro may be used to configure logging. If it is not called. The default LogConfig is used. By default logging is only done to stdout. A sample log_config! call might look something like this:"],["log_config_multi","log_config_multi is identical to [`log_config`] except that the name of the logger is specified instead of using the default logger."],["log_multi","log_multi is identical to [`log`] except that the name of the logger is specified instead of using the default logger."],["log_no_ts","Log using the default logger and don’t print a timestamp. See [`log`] for more details on logging."],["log_no_ts_multi","Identical to [`log_no_ts`] except that the name of the logger is specified instead of using the default logger."],["trace","Log at the ‘trace’ (0) log level. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. Also see [`debug`], [`info`], [`warn`], [`error`], or [`fatal`]."],["trace_no_ts","Just like [`trace`], but with no timestamp."],["warn","Log at the ‘warn’ (3) log level. This macro calls the default logger. To configure this logger, see [`log_config`]. It is used like the pritln/format macros. Also see [`trace`], [`debug`], [`info`], [`error`], or [`fatal`]."],["warn_no_ts","Just like [`warn`], but with no timestamp."]],"mod":[["logger","A logging library."]]});