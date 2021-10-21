# binary-install

Install .tar.gz binary applications via npm

## Usage

This library provides a single class `Binary` that takes the name of your binary, and download url.

```js
let binary = new Binary('my-binary', 'https://example.com/binary/tar.gz')
```

After your `Binary` has been created, you can run `.install()` to install the binary, and `.run()` to run it.

`install()` takes an optional argument of additional options to pass to Axios when downloading the binary. This is useful for downloading from a private repo when you have to set an `Authorization` header.

### Example

This is meant to be used as a library - create your `Binary` with your desired options, then call `.install()` in the `postinstall` of your `package.json`, and `.run()` in the `bin` section of your `package.json`. See [this example project](/packages/binary-install-example) to see how to create an npm package that installs and runs a binary using the GitHub releases API.
