use walkdir::WalkDir;

fn main() {
    let mut build = cc::Build::new();
    let env = std::env::var("TARGET").unwrap();

    // windows includes
    if env.contains("windows") {
        build.include("bx/include/compat/msvc");
        build.include("bgfx/3rdparty/directx-headers/include/directx");
        build.flag("/Zc:__cplusplus");
        build.flag("/std:c++17");
    } else if env.contains("darwin") {
        // macOS includes
        build.include("bx/include/compat/osx");
        build.flag("-std=c++14");
    } else if env.contains("emscripten") {
        build.flag("-std=c++14");
    }

    // add shared include dirs
    build.include("bgfx/3rdparty/khronos");
    build.include("bgfx/3rdparty/glsl-optimizer/src/glsl");
    build.include("bgfx/3rdparty/glsl-optimizer/src/util");
    build.include("bgfx/3rdparty/glsl-optimizer/src/mesa");
    build.include("bgfx/3rdparty/glsl-optimizer/src");
    build.include("bgfx/3rdparty/glsl-optimizer/include");
    build.include("bgfx/3rdparty/glslang/glslang/Public");
    build.include("bgfx/3rdparty/glslang");
    build.include("bgfx/3rdparty/fcpp");
    build.include("bgfx/3rdparty/spirv-cross");
    build.include("bgfx/3rdparty/spirv-tools/include");
    build.include("bgfx/3rdparty/spirv-tools/include/generated");
    build.include("bgfx/3rdparty/spirv-tools");
    build.include("bgfx/3rdparty/spirv-headers/include");
    build.include("bgfx/3rdparty/webgpu/include");
    build.include("bgfx/3rdparty");
    build.include("bgfx/include");
    build.include("bx/include");
    build.include("bx/3rdparty");
    build.include("bimg/include");

    // defines - Currently not supporting WebGPU, GNM
    // OS support:
    // Windows - DX11
    // macOS - Metal
    // Posix - OpenGL
    // Android - OpenGL/Vulkan
    // Emscripten - OpenGLES
    // In the future it would be good to make this configurable instead

    build.define("BGFX_CONFIG_RENDERER_WEBGPU", "0");
    build.define("BGFX_CONFIG_RENDERER_GNM", "0");

    build.define("BX_CONFIG_DEBUG", "0");
    build.define("ENABLE_OPT", "1");

    if env.contains("windows") {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_DIRECT3D11", "1");
        build.define("BGFX_CONFIG_RENDERER_DIRECT3D12", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "1");
        build.define("_WIN32", None);
        build.define("_HAS_EXCEPTIONS", "0");
        build.define("_SCL_SECURE", "0");
        build.define("_SECURE_SCL", "0");
        build.define("__STDC_LIMIT_MACROS", None);
        build.define("__STDC_FORMAT_MACROS", None);
        build.define("__STDC_CONSTANT_MACROS", None);
        build.define("_CRT_SECURE_NO_WARNINGS", None);
        build.define("_CRT_SECURE_NO_DEPRECATE", None);
        build.warnings(false);
    } else if env.contains("darwin") {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "0");
        build.define("BGFX_CONFIG_RENDERER_METAL", "1");
    } else if env.contains("android") {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGLES", "1");
    } else if env.contains("emscripten") {
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "0");
        build.define("BGFX_CONFIG_RENDERER_OPENGLES", "1");
    } else {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "1");
    }

    // sources
    build.file("bx/src/amalgamated.cpp");
    build.file("bgfx/src/vertexlayout.cpp");
    build.file("bgfx/src/shader.cpp");
    build.file("bgfx/src/shader_dx9bc.cpp");
    build.file("bgfx/src/shader_dxbc.cpp");
    build.file("bgfx/src/shader_spirv.cpp");
    build.file("bgfx/tools/shaderc/shaderc_glsl.cpp");
    build.file("bgfx/tools/shaderc/shaderc_hlsl.cpp");
    build.file("bgfx/tools/shaderc/shaderc_metal.cpp");
    build.file("bgfx/tools/shaderc/shaderc_pssl.cpp");
    build.file("bgfx/tools/shaderc/shaderc_spirv.cpp");
    build.file("bgfx/tools/shaderc/shaderc.cpp");

    for entry in WalkDir::new("bgfx/3rdparty").into_iter().flatten() {
        let Some(ext) = entry.path().extension().map(|e| e.to_string_lossy()) else {
            continue;
        };

        let name = entry
            .path()
            .as_os_str()
            .to_string_lossy()
            .replace("\\", "/");

        let banned_files = [
            "android_native_app_glue.c",
            "HLSL/hlslAttributes.cpp",
            "HLSL/hlslGrammar.cpp",
            "HLSL/hlslOpMap.cpp",
            "HLSL/hlslParseables.cpp",
            "HLSL/hlslParseHelper.cpp",
            "HLSL/hlslScanContext.cpp",
            "HLSL/hlslTokenStream.cpp",
            "node/binding.cpp",
            "node/compiler.cpp",
            "node/shader.cpp",
            "spirv-cross/main.cpp",
            "spirv-remap.cpp",
            "StandAlone.cpp",
            "Unix/ossource.cpp",
            "usecpp.c",
            "glsl/main.cpp",
            "getopt/getopt_long.c",
        ];

        if (ext == "cpp" || ext == "c") && !banned_files.iter().any(|b| name.ends_with(b)) {
            build.file(entry.path());
        }
    }

    build.compile("bgfx_shaderc");
}
