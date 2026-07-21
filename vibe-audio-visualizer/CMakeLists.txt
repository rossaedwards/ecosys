###############################################################################
# CMakeLists.txt — Vibe Audio Visualizer VLC Plugin
# Aurphyx LLC & Aurphyx SUXS
# V.A.P. v3.1 — 9-Pillar TSLCA Cymatic Engine
###############################################################################

cmake_minimum_required(VERSION 3.18)
project(vibe_audio_visualizer VERSION 1.0.0 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# ── Build type default ────────────────────────────────────────────────────
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

# ── Find VLC ─────────────────────────────────────────────────────────────
find_package(PkgConfig REQUIRED)
pkg_check_modules(VLC REQUIRED libvlc vlc-plugin)

if(NOT VLC_FOUND)
    message(FATAL_ERROR
        "VLC development headers not found.\n"
        "Windows: vcpkg install vlc  or set VLC_INCLUDE_DIR manually.\n"
        "Fedora:  sudo dnf install vlc-devel\n"
        "Ubuntu:  sudo apt install libvlc-dev vlc-plugin-base")
endif()

# ── Find OpenGL & math ───────────────────────────────────────────────────
find_package(OpenGL REQUIRED)

# ── Find kissfft (bundled in VLC or system) ──────────────────────────────
# VLC ships kissfft internally; if building standalone, provide path.
find_path(KISSFFT_INCLUDE_DIR kiss_fft.h
    HINTS ${VLC_INCLUDE_DIRS} /usr/include/kissfft /usr/local/include/kissfft)

find_library(KISSFFT_LIB
    NAMES kissfft kissfft-float
    HINTS /usr/lib /usr/local/lib)

if(NOT KISSFFT_INCLUDE_DIR)
    message(STATUS "kissfft headers not found externally — "
                   "using VLC's bundled kissfft (recommended)")
endif()

# ── Source files ─────────────────────────────────────────────────────────
set(VAV_SOURCES
    src/vibe_visualizer.c    # VLC plugin entry — Open/Close/DoWork
    src/vap_runtime.c        # VAP 9-pillar runtime state
    src/vap_loader.c         # Sidecar / ID3 / Vorbis loader
    src/dsp_engine.c         # Phase I: FFT + onset + chromatic bands
    src/gl_renderer.c        # OpenGL 2.1 two-pass render pipeline
)

set(VAV_HEADERS
    src/vap_runtime.h
    src/vap_loader.h
    src/dsp_engine.h
    src/gl_renderer.h
    vap/vap_photometric.h
    vap/vap_affective.h
)

# ── VLC plugin shared library target ────────────────────────────────────
# VLC requires the output file to be named exactly:
#   <module_name>_plugin.dll  (Windows)
#   lib<module_name>_plugin.so  (Linux)
add_library(vibe_visualizer_plugin MODULE ${VAV_SOURCES} ${VAV_HEADERS})

# ── Include paths ────────────────────────────────────────────────────────
target_include_directories(vibe_visualizer_plugin PRIVATE
    ${VLC_INCLUDE_DIRS}
    ${OPENGL_INCLUDE_DIRS}
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/vap
    $<$<BOOL:${KISSFFT_INCLUDE_DIR}>:${KISSFFT_INCLUDE_DIR}>
)

# ── Compile definitions ──────────────────────────────────────────────────
target_compile_definitions(vibe_visualizer_plugin PRIVATE
    MODULE_STRING="${CMAKE_PROJECT_NAME}"
    PACKAGE_VERSION="${PROJECT_VERSION}"
    HAVE_CONFIG_H
    $<$<BOOL:${KISSFFT_INCLUDE_DIR}>:HAVE_KISSFFT>
)

# ── Compiler flags ───────────────────────────────────────────────────────
target_compile_options(vibe_visualizer_plugin PRIVATE
    $<$<C_COMPILER_ID:GNU,Clang>:
        -Wall -Wextra -Wno-unused-parameter
        -fvisibility=hidden
        -ffast-math
        -O3
    >
    $<$<C_COMPILER_ID:MSVC>:
        /W3 /O2 /fp:fast
    >
)

# ── Link libraries ───────────────────────────────────────────────────────
target_link_libraries(vibe_visualizer_plugin PRIVATE
    ${VLC_LIBRARIES}
    ${OPENGL_LIBRARIES}
    m   # libm — math.h (sinf, cosf, sqrtf, expf)
    $<$<BOOL:${KISSFFT_LIB}>:${KISSFFT_LIB}>
)

# ── Platform-specific linker flags ───────────────────────────────────────
if(WIN32)
    target_link_options(vibe_visualizer_plugin PRIVATE
        /DEF:${CMAKE_SOURCE_DIR}/src/vibe_visualizer.def)
    set_target_properties(vibe_visualizer_plugin PROPERTIES
        PREFIX ""
        SUFFIX "_plugin.dll")
elseif(APPLE)
    set_target_properties(vibe_visualizer_plugin PROPERTIES
        PREFIX "lib"
        SUFFIX "_plugin.dylib")
else()
    # Linux / Fedora
    set_target_properties(vibe_visualizer_plugin PROPERTIES
        PREFIX "lib"
        SUFFIX "_plugin.so")
endif()

# ── Shader files — copy to build dir so gl_renderer can load from disk ──
file(COPY ${CMAKE_SOURCE_DIR}/shaders
     DESTINATION ${CMAKE_BINARY_DIR})

# ── Install target ───────────────────────────────────────────────────────
if(WIN32)
    # Typical VLC Windows install path
    set(VLC_PLUGIN_DIR
        "C:/Program Files/VideoLAN/VLC/plugins/visualization"
        CACHE PATH "VLC visualization plugin directory")
elseif(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    # Fedora 44 — your rig
    execute_process(
        COMMAND pkg-config --variable=pluginsdir vlc-plugin
        OUTPUT_VARIABLE VLC_PLUGIN_DIR
        OUTPUT_STRIP_TRAILING_WHITESPACE)
    if(NOT VLC_PLUGIN_DIR)
        set(VLC_PLUGIN_DIR "/usr/lib64/vlc/plugins/visualization")
    else()
        set(VLC_PLUGIN_DIR "${VLC_PLUGIN_DIR}/visualization")
    endif()
endif()

install(TARGETS vibe_visualizer_plugin
        LIBRARY DESTINATION ${VLC_PLUGIN_DIR}
        RUNTIME DESTINATION ${VLC_PLUGIN_DIR})

install(DIRECTORY shaders
        DESTINATION ${VLC_PLUGIN_DIR}/vibe_shaders)

message(STATUS "")
message(STATUS "═══════════════════════════════════════════════")
message(STATUS " Vibe Audio Visualizer — V.A.P. v3.1")
message(STATUS " Build type  : ${CMAKE_BUILD_TYPE}")
message(STATUS " VLC headers : ${VLC_INCLUDE_DIRS}")
message(STATUS " Install dir : ${VLC_PLUGIN_DIR}")
message(STATUS " Bloom pass  : ON (arousal × brightness_ceiling)")
message(STATUS " Pillars     : 9 (TSLCA)")
message(STATUS "═══════════════════════════════════════════════")
message(STATUS "")
