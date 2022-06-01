***

<h1 align="center">
<code>HTTPB</code>
</h1>
<p align="center">
Hypertext Transfer Protocol Broken
</p>

***

An experimental project to use UDP for a webserver. As HTTP is built on TCP, there needs to be an interface through which the requests and responses are sent so that browsers can render the webpage, but the actual part of handling the request and returning the response is done by an UDP socket.