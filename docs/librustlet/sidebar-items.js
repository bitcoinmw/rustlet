initSidebarItems({"macro":[["add_header","Adds a header to the response for this rustlet. The first parameter is the name of the header to set and the second parameter is the value of the header. See examples below."],["async_complete","Completes a rustlet execution that is executing in an async_context. This macro must be called so that the rustlet container knows that a particular async rustlet is complete. Also see [`async_context`] and the example below."],["async_context","Creates an async context which may be used to pass execution of the rustlet into another thread thus allowing the rustlet thread pool to continue processing other rustlets. The first call to [`async_context`] should not specify any parameters and the second call should specify the returned parameter of the first call and be in another thread. See the example below for details."],["bin_write","Writes a binary response to the client. The parameter must be a byte array. Note that: data written via bin_write is buffered and is not necessarily sent immidiately. To ensure all data is written, the user must call the [`flush`] macro."],["cookie","Get the value of the specified cookie. To set cookies, see [`set_cookie`]."],["flush","Flushes any buffered data previously sent via the [`response`] macro."],["header_len","Returns the number of headers sent in this HTTP request."],["header_name","Returns the header name for the specified index."],["header_value","Returns the header value for the specified index."],["mainlogerror","Internal macro used to log to the main log. Applications should use the default logger (or another user specified logger). See [`nioruntime_log`] for details on logging."],["query","Get the value of the specified query parameter. Parsing is done with the `querystring` library."],["request","Get data from the request for this rustlet. See the example below for possible values of the request parameter."],["request_content","Returns the content of the message body of the HTTP request."],["response","Writes a formated response to the client. The formatting is the same formatting as the format! macro, as that is used internally to format the response. Note that data written via response is buffered and is not necessarily sent immidiately. To ensure all data is written, the user must call the [`flush`] macro."],["rustlet","Specifies a rustlet. Rustlets are closures that process HTTP requests and generate a response, so variables can be moved into them and shared among other rustlets or any other closure. Rustlets are processed in the nioruntime. So, the exectuion is performant. See the other macros for detailed examples on how to use all of the functionality of rustlets. To try out a minimally simple rustlet as a project, see rustlet-simple."],["rustlet_init","Initialize the rustlet container based on the specified configuration. The default configuration may be used by calling `RustletConfig::default()`. See [`crate::RustletConfig`] for details on configuring the Rustlet and Http containers."],["rustlet_mapping","Maps the specified uri to a rustlet. All requests to the container for this uri will be processed by the specified rustlet."],["session","Sets or gets a value in the session. The session is an in-memory key/value store that can be used by rustlets to store data that can be accessed by other rustlets. A session cookie is set called rustletsessionid that lets the rustlet container know which user is which. The session is automatically invalidated after a certain period of time where no calls to session! or session_delete! are made. By default, this amount of time is 30 minutes, but it is configurable in [`crate::RustletConfig::session_timeout`]. If only one parameter is specified, the value is retrieved from the session data store, if two parameters are specified, the value is set, see the examples below for more details."],["session_delete","Delete the session. See [`session`] for more information on sessions. If a parameter is specified, only that parameter is deleted from the session. With no parameter specified, the entire session is invalidated."],["set_content_type","Sets the content-type header of this request."],["set_cookie","Set the value of the specified cookie. To get cookies, see [`cookie`]."],["set_redirect","Sets a redirect to another URL. The 301 HTTP Response is used for the redirect. See example below."]],"mod":[["macros",""],["rustlet_impls",""]],"struct":[["Error","Base Error struct which is used throught this crate and other crates"],["HttpConfig","The configuration struct for an [`HttpServer`]. The [`Default`] trait is implemented for this struct so default can be used like below."]],"trait":[["Readable","Trait that every type that can be deserialized from binary must implement. Reads directly to a Reader, a utility type thinly wrapping an underlying Read implementation."],["Reader","Implementations defined how different numbers and binary structures are read from an underlying stream or container (depending on implementation)."],["Writeable","Trait that every type that can be serialized as binary must implement. Writes directly to a Writer, a utility type thinly wrapping an underlying Write implementation."],["Writer",""]]});