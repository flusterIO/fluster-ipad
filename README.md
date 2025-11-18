# Fluster iOS

![Fluster screenshot](shared_assets/logo/fluster_banner_with_name.png)

> Checkout our website [here](https://flusterapp.com).

After building a cross platform desktop version of [Fluster](https://github.com/flusterIO/fluster/releases) I've decided to focus solely on the Apple ecosystem for now by rebuilding the core of the application yet again, this time in Swift to integrate all of Apple's native features with the application. This deep integration with Apple's incredible library of tools and API's given to developers just wasn't possible with cross platform libraries, as amazing as [Tauri](https://v2.tauri.app/) is.

The goal is to maintain full mdx support, while integrating Apple's amazing pencilkit to allow full hand-drawn documents alongside any mdx documents. From there, the mdx content will be fed to onboard AI models for various tasks. Unfortunately, passing the hand drawn content to language models would require quite a bit of finagling and will take some time, but using the mdx content alongside hand drawn content, while linking the two together seamlessly, we can provide a full set of AI features while providing the structured output that mdx provides, while still allowing the free-flow thinking that hand drawn documents give.

This app will hopefully be ready for release in early 2026.
