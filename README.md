# Bevy Skeleton POC

Under development

Goals:

* Use character from mixamo.com as gltf asset
* Allow camera movement around character
* Allow individual mouse-based bone/joint dragging

Issues encountered:

* Failed loading standard GLTF
  * Output:

      ```
      thread 'main' panicked at 'Attribute Vertex_Uv is required by shader, but not supplied by mesh. Either remove the attribute from the shader or supply the attribute (Vertex_Uv) to the mesh.'
      ```

  * Issue - https://github.com/bevyengine/bevy/issues/1785
  * Workaround - Load in blender and export w/ different values as suggested in issue
* Warning with shader
  * Output:

      ```
      WARN Device::create_shader_module: wgpu_core::device: Failed to parse shader SPIR-V code: UnsupportedInstruction(Function, LogicalAnd)
      WARN Device::create_shader_module: wgpu_core::device: Shader module will not be validated or reflected
      ```

  * Issue - https://github.com/bevyengine/bevy/issues/1812
  * Workaround - reference patched bevy locally as suggested in issue (but not doing, just a warning)