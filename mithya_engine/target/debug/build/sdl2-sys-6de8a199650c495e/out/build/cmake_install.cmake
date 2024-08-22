# Install script for directory: C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/Release/SDL2main.lib")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/Release/SDL2-static.lib")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/Release/SDL2_test.lib")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2mainTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2mainTargets.cmake"
         "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2mainTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2mainTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2mainTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2mainTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2mainTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2staticTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2staticTargets.cmake"
         "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2staticTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2staticTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2staticTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2staticTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2staticTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2testTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2testTargets.cmake"
         "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2testTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2testTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/cmake/SDL2testTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2testTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/CMakeFiles/Export/272ceadb8458515b2ae4b5630a6029cc/SDL2testTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Devel" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/cmake" TYPE FILE FILES
    "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/SDL2Config.cmake"
    "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/SDL2ConfigVersion.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SDL2" TYPE FILE FILES
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_assert.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_atomic.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_audio.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_bits.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_blendmode.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_clipboard.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_copying.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_cpuinfo.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_egl.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_endian.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_error.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_events.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_filesystem.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_gamecontroller.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_gesture.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_guid.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_haptic.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_hidapi.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_hints.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_joystick.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_keyboard.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_keycode.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_loadso.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_locale.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_log.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_main.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_messagebox.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_metal.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_misc.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_mouse.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_mutex.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_name.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengl.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengl_glext.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_gl2.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_gl2ext.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_gl2platform.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_khrplatform.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_pixels.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_platform.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_power.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_quit.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_rect.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_render.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_rwops.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_scancode.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_sensor.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_shape.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_stdinc.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_surface.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_system.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_syswm.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_assert.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_common.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_compare.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_crc32.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_font.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_fuzzer.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_harness.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_images.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_log.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_md5.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_memory.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_random.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_thread.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_timer.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_touch.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_types.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_version.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_video.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_vulkan.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/begin_code.h"
      "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/close_code.h"
      "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/include/SDL_revision.h"
      "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/include-config-release/SDL_config.h"
      )
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/licenses/SDL2" TYPE FILE FILES "C:/Users/nisha/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/LICENSE.txt")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/sdl2.pc")
endif()

if(CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_COMPONENT MATCHES "^[a-zA-Z0-9_.+-]+$")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
  else()
    string(MD5 CMAKE_INST_COMP_HASH "${CMAKE_INSTALL_COMPONENT}")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INST_COMP_HASH}.txt")
    unset(CMAKE_INST_COMP_HASH)
  endif()
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
  file(WRITE "C:/Programming Workbench/Rust/MithyaEngine/mithya_engine/target/debug/build/sdl2-sys-6de8a199650c495e/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
