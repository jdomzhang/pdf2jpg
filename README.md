# pdf2jpg

## pdfium

- [binary files](https://github.com/bblanchon/pdfium-binaries/releases/tag/chromium%2F6164)

## cross-compile

### target

```bash
rustup target list
rustup target add x86_64-unknown-linux-musl
```

### build on macos for linux

```bash
TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
```

## install on mac

```bash
sudo wget -q https://cdn.edesoft.com/tools/pdf2jpg/macos/pdf2jpg -O /usr/local/bin/pdf2jpg
sudo chmod 777 /usr/local/bin/pdf2jpg
sudo wget -q https://cdn.edesoft.com/tools/pdf2jpg/macos/libpdfium.dylib -O /usr/local/lib/libpdfium.dylib
```

## install on linux

```bash
sudo wget -q https://cdn.edesoft.com/tools/pdf2jpg/linux/pdf2jpg -O /usr/local/bin/pdf2jpg
sudo chmod 777 /usr/local/bin/pdf2jpg
sudo wget -q https://cdn.edesoft.com/tools/pdf2jpg/linux/libpdfium.so -O /usr/local/lib/libpdfium.so
sudo chmod 777 /usr/local/lib/libpdfium.so
```
