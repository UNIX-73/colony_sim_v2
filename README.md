# Chunk-Based World Simulator (Rust)

This project is a Rust-based experimental engine for managing large voxel-like worlds divided into chunks. Originally started as a colony simulation, it evolved into a multithreaded infrastructure for spatial data management, focused on performance and extensibility.

## Overview

![Chunk simulation demo](example.gif)

The engine is built around a chunk-based system where the world is divided into equally sized 3D regions. Each chunk handles its internal data and is optimized for memory and performance through compression, caching, and smart rendering strategies.

Although currently minimal in gameplay, it serves as a technical prototype for managing large, dynamic voxel spaces.

## Features

- **Chunked world architecture** for scalable simulation
- **Multithread-safe data access**: chunks are decoupled for parallel updates
- **RLE compression** to reduce memory usage of chunk data
- **LRU cache system** for disk-backed chunk storage (swap in/out)
- **Occlusion culling**: avoids rendering fully enclosed blocks
- **Bevy engine** used for rendering (WGPU backend)
- Modular system ready to be extended with gameplay logic

## Technical Highlights

- **Written in Rust** with strong emphasis on ownership and thread safety
- **Rendering with Bevy**
- Chunk visibility is computed per-face based on neighboring blocks
- Compressed chunks are only decompressed on demand for editing or rendering
- Designed to support streaming terrain or simulation data

## Status

This project is in a prototype state. Rendering and core systems are working, but no gameplay or UI is currently implemented. Its main purpose is experimentation with chunk compression, access synchronization, and performance optimization in Rust.

## Future Plans

- Add gameplay systems (e.g. agent simulation, terrain generation, temperature)
- Support for persistent storage (e.g. save/load worlds to disk)
- Interactive tools for terrain editing

## Known Issues

- Chunk borders do not get unloaded from rendering until they are evicted from the LRU cache
