# broken-links

Find all the broken links in your copy!

`broken-links` will 

1. Take the output of `git ls-files` 
2. Search all the files mentioned for anything that looks like a link
3. Perform a `get` request and print out the status code

## Build from source:

Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```
curl https://sh.rustup.rs -sSf | sh
```

Create binary

```
cargo build
```

This will create a binary and place it in `target/release`. You can add this location to your `$PATH` or copy this binary to a location already in your `$PATH`, or just move it to the repository you want check.

Goto the directory you want to test and:

```
[~/Documents/parthmehrotra.com, master]: broken-links
Pass: http://code.jquery.com/jquery-1.10.2.min.js => 200 OK
Pass: http://netdna.bootstrapcdn.com/bootstrap/3.0.3/js/bootstrap.min.js => 200 OK
Pass: https://www.github.com/parth => 200 OK
Pass: https://www.instagram.com/mehrotraparth/ => 200 OK
Pass: https://gemini.com/ => 200 OK
Pass: https://www.jpmorgan.com/country/US/EN/about/asset-management => 200 OK
Pass: https://cs.rutgers.edu => 200 OK
Pass: http://go.sap.com/index.html => 200 OK
Pass: https://www.youtube.com/watch?v=UgDz_9i2nwc => 200 OK
Pass: https://www.youtube.com/embed/UgDz_9i2nwc => 200 OK
Pass: https://github.com/parth/StaticMapGenerator => 200 OK
Pass: https://en.wikipedia.org/wiki/Mercator_projection#Mathematics_of_the_Mercator_projection => 200 OK
Pass: https://en.wikipedia.org/wiki/Mercator_projection#Mathematics_of_the_Mercator_projection => 200 OK
Fail: http://quicksetup.me => http://quicksetup.me/: timed out
Fail: http://quicksetup.me => http://quicksetup.me/: timed out
Pass: h`ttp://go.sap.com/index.html => 200 OK
Fail: http://autoai.org => http://autoai.org/: timed out
Pass: http://autoai.org => 200 OK
Pass: http://flask.pocoo.org/ => 200 OK
Pass: http://www.celeryproject.org/ => 200 OK
Pass: https://cloud.google.com/appengine/docs => 200 OK
Pass: http://www.rutgers.edu/ => 200 OK
Pass: https://www.cs.rutgers.edu/ => 200 OK
Fail: http://wh.wayneschools.com/ => http://wh.wayneschools.com/: timed out
```
