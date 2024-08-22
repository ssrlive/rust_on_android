# how to import rust code into android project

Just a demo, see the [commit #4](https://github.com/ssrlive/rust_on_android/commit/7fa92cd01b24258469ed173a33b593e8d472fe99) for the details.


## Android Studio

* Android Studio
* Android SDK Platform 31
* Android NDK 24.0.8215888
* Android SDK Build-Tools 31.0.0
* Android SDK Command-line Tools
* Android SDK Platform-Tools

#### Config: Tools >> SDK Manager >>  SDK Tools (middle tab):

![image](https://user-images.githubusercontent.com/30760636/200150722-e48dae21-51d0-4993-a8b7-95ea0330249c.png)

## Rust

Install rust on your PC from [rustup](https://rustup.rs), 
then add some Android targets (arm64, arm, x86_64, x86) for rust.
```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```
Uses [rust-android-gradle](https://github.com/mozilla/rust-android-gradle) plugin, so is built with the command:
```cli
gradlew cargoBuild

# build release version
gradlew assembleRelease
```

> Set `JAVA_HOME` environment variable
> ```
> Windows:  set JAVA_HOME="C:\Program Files\Android\Android Studio\jbr"
> macOS:    export JAVA_HOME=/Applications/Android\ Studio.app/Contents/jre/Contents/Home/
> Linux:    (needn't to anything, using the system settings.)
> ```

### Function naming convention

In `src/lib.rs` you need to name the function according to the following naming convention in order to make it available in `Java`.

If the _Java_ function is called `greeting` and it is saved in a file named `RustBindings.java` pulled from package `com.example.myrustapp` then in _Rust_ the function name is:

| Java |     package name      |   filename   | function name |
| :--: | :-------------------: | :----------: | :-----------: |
| Java | com_example_myrustapp | RustBindings |   greeting    |

Which would look like this:

`Java_com_example_myrustapp_RustBindings_greeting(...)`

## Python

Install [Python](https://www.python.org/downloads/) on your PC.

> In `macOS` Monterey 12.3 and above, python was removed by Apple, you must install [Python3](https://www.python.org/downloads/) by yourself, then run this command to make python3 as python.
> ```
> ln -s -f /usr/local/bin/python3 /usr/local/bin/python
> ```
