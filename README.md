# Introduction
The goal of this repo is to provide examples of how to use Rust to compile to Web Assembly.

## Motivation

I recently completed a hack-a-thon where my team made a Rust Javascript AST parser, which compiled to Web Assembly. I found several pieces of information on the web, but everything felt disconnected. This repo aims to bring together the information I learned so that others can compile to Web Assembly.

## Getting Started

*Assumptions:*
	* You have a basic understand of Rust and Web Languages (HTML/JS)
	* You have Rust installed

### Web Assembly Target

You'll need to add the Web Assembly target, so that Rust can compile to it. Run this command in your terminal:

`rustup target add wasm32-unknown-emscripten`

### Emscripten SDK + tool chain

Next, you need to install the emscripten tool chain. You can either compile it from source, or install the portable. Download the portable version from: http://kripken.github.io/emscripten-site/docs/getting_started/downloads.html#download-and-install

Extract it some where, and execute `source ./emsdk_env.sh` against the included `emsdk_env.sh`. 

This should cause `emcc` to become available to your terminal. Test this by reading the version string: `emcc -v`, if it doesn't appear, you may need to update your `$PATH` to include the path to the extracted emsdk.

Now we need install the toolchain.

Update the list of available binaries: `emsdk update`
Install latest: `emsdk install latest`
Activate latest: `emsdk activate latest`

If you don't want latest, or want to install a different version, first execute `emsdk list` to get a list of available binaries, then install the matching version.

### Ready to compile

#### rustc

When compiling with EMSDK using `rustc`, you'll generate an example HTML file, and a Module Javascript file in addition to the WASM. The following example presumes you have a "hello world" rust file.

```
rustc --target=wasm32-unknown-emscripten hello.rs -o hello.html
Generates::

hello.wasm
hello.js
hello.html
```

#### cargo build

Compiling using `cargo build` is a bit more complex. The following examples presumes you have a "hello world" cargo project, i.e, `cargo new hello-world`.

```
cargo build --release --target wasm32-unknown-emscripten
```

This will create a release with the WASM target. Unlike the `rustc` example, this will only generate the `.wasm` and `.js` file (amongst other cargo build files).

You'll need to create an HTML file which includes the generated script. *Note:* the generated JS file assumes the WASM is in the same directory as the JS file.

```
# index.html
<html>
	<head>Hello-Cargo</head>
	<body>
		<script type="text/javascript" src="./hello-cargo.js"></script>
		<script type="text/javascript">
			Module.addOnInit(function () {
				console.log('Module ready!');
			});
		</script>
	</body>
</html>
```

### Executing methods in rust

Now lets do something meaningful, and execute a method defined in our rust application.

Lets say that you have a string, "One apon a time" and you want to replace "One" with "Once".

```
main.rs

use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

fn my_string_safe(i: *mut c_char) -> String {
  unsafe {
      CStr::from_ptr(i).to_string_lossy().into_owned()
  }
}

#[no_mangle]
pub fn fix_story(in: *mut c_char) -> *mut c_char {
	let data = my_string_safe(in);
	let out = data.replace("one", "once");

	CString::new(out.as_str())
		.unwrap()
		.into_raw()
}
```
```
.cargo/config

[target.wasm32-unknown-emscripten]
rustflags = [
    "-Clink-args=-s EXPORTED_FUNCTIONS=['_fix_story'] -s ASSERTIONS=1",
]
```
```
index.html

<script type="text/javascript">
	Module.addOnInit(function () {
		var story = "One apon a time";
		// Execute `cwrap`(https://kripken.github.io/emscripten-site/docs/api_reference/preamble.js.html#cwrap)
		// This gives us a function that will store the string in memory, and pass that pointer to the binary
		// Parameters ('rust_fn_name', 'expected_return_type', ['parameter type'])
		var fix_story = Module.cwrap('fix_story', 'string', ['string']);
		console.log(fix_story(str)); // Once apon a time
	});
</script>
```

#### So how does this work?

In rust, we declare that we expect a raw pointer defined as a `c_char` type. We use CStr to access that pointer, and convert it to a std::String. Finally, we use CString to convert the std::String back into a `c_char` pointer and return that back to Web Assembly. emscripten does the work (within `cwrap`) to capture the string from memory, and return it to the JS runtime (and ultimately `console.log` it).

#### How does Web Assembly know how to access `fix_story` ?

In the .cargo/config file we define a set of exported functions to the compiler via `EXPORT_FUNCTIONS`. *Note:* You must prepend the exported function with an underscore.

## Examples

I've included several examples under the `examples` directory. Just `cd` into one, and run `make` to build the example.

*Note:* You will need a server to host the released files. I personally use `serve` which can be found over at https://www.npmjs.com/package/serve.

## Todo

[x] Create string -> String example
[x] Create number -> i32 example
[ ] Create array -> vec example
[ ] Create object -> struct example _possible?_

