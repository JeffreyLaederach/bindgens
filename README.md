# bindgens

Rust FFI Bindings for .NET Languages (C#, F#, VB.NET) and C/C++  

This repo provides a useful way to automatically generate C#, C, and C++ bindings from Rust functions via Rust's Foreign Function Interface (FFI) and the `csbindgen` + `cbindgen` crates.

### Setup:

Place your Rust functions/code in the `lib.rs` file as shown in the example below:

```Rust
// Rust FFI (Foreign Function Interface)

#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}
```

Next, go to the `build.rs` file to customize the output files you would like to generate:

```Rust
use std::env;

fn main() {
    // For Generating C# Bindings:
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("bindgens")
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();

        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        // For Generating C Bindings:
        cbindgen::Builder::new()
          .with_crate(crate_dir)
          .generate()
          .expect("Unable to generate bindings")
          .write_to_file("../c/bindings.h");

        // For Generating C++ Bindings:
        cbindgen::Builder::new()
          .with_crate(crate_dir)
          .generate()
          .expect("Unable to generate bindings")
          .write_to_file("../cpp/bindings.hpp");
}
```

Here you can change the folder directory and file name/extension of the output files in the three different languages. You can comment out any section for a language you do not want to create bindings for.  

**Note:** For C/C++ you must also go to the `cbindgen.toml` file to further customize which language your bindings will be generated in. You can also add many other parameters to format the output here as well:  

```Toml
# The language to output bindings in
#
# possible values: "C", "C++", "Cython"
#
# default: "C++"

# Uncomment on of the following lines to choose Language (C/C++)
language = "C"
# language = "C++"

namespace = "bindings"
```

In order to generate the bindings, simply go to the `build.rs` file and save it while in VS Code with Rust Analyzer activated and it will execute. If not, use `cargo build` while in the `bindgens` library directory.

### Outputs:

The following is an example of generating a C# file from Rust using csbindgen. This file can be called from F# or Visual Basic .NET when implemented in a Visual Studio solution/project:

```C#
// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;

namespace CsBindgen
{
    internal static unsafe partial class NativeMethods
    {
        const string __DllName = "bindgens";

        [DllImport(__DllName, EntryPoint = "my_add", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern int my_add(int x, int y);
    }
}
```

To call this code from any .NET programming languages, you must first place it inside a C# project and then add a project reference to it inside the `.fsproj` or `.vbproj` files for F# or VB.NET projects you are trying to call this code from.  

**Calling the C# function from F#**

```F#
open System
open CsBindgen

[<EntryPoint>]
let main argv =
    let result = NativeMethods.my_add(2, 3)
    printfn "Result: %d" result
    0
```

**Calling the C# function from VB.NET**

```VB.NET
Imports System
Imports CsBindgen

Module Program
    Sub Main()
        Dim result As Integer = NativeMethods.my_add(2, 3)
        Console.WriteLine("Result: {0}", result)
    End Sub
End Module
```

For C/C++, cbindgen generates header files with the standard libraries imported at the top as shown below:

```C
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t my_add(int32_t x, int32_t y);

}  // extern "C"

```
