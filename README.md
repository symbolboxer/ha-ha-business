# ha-ha-business

Scrapes business information from a certain card game. Written in Rust because I wanted
to toy around with it. This is engineered at a standard suitable for my rinky-dink
project, so use at your own risk.

## A small warning

This is written using reqwest's blocking API so that someone using it doesn't look like
they're DDOSing the site. Should you decide to convert this to asynchronous, be sure
that you throttle your requests.
