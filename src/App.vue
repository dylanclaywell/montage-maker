<script setup lang="ts">
import { ref, watch } from 'vue'

const uploadMethod = ref('image')
const selectedFolder = ref('')
const selectedImages = ref<string[]>([])

function onFolderChange(e: Event) {
  const target = e.target as HTMLInputElement
  const files = target.files as FileList
  const file = files[0]
  selectedFolder.value = file.name
}

function onImageUpload(e: Event) {
  const target = e.target as HTMLInputElement
  const files = target.files as FileList
  const file = files[0]
  selectedImages.value.push(file.name)
}

watch(uploadMethod, (newVal) => {
  if (newVal === 'folder') {
    selectedImages.value = []
  } else {
    selectedFolder.value = ''
  }
})
</script>

<template>
  <div>
    <h1>Montage Maker</h1>
    <p>Upload your images and create a montage</p>

    <div class="grid grid-cols-1 md:grid-cols-2">
      <div class="flex flex-col">
        <fieldset>
          <legend>Upload Method</legend>
          <label>
            <input
              name="upload-method"
              type="radio"
              v-model="uploadMethod"
              value="image"
            />
            <span>Image</span>
          </label>
          <label>
            <input
              name="upload-method"
              type="radio"
              v-model="uploadMethod"
              value="folder"
            />
            <span>Folder</span>
          </label>
        </fieldset>
        <div v-if="uploadMethod === 'folder'">
          <p>
            Selecting a folder will attempt to combine all images in the given
            directory. Make sure the directory only includes the files you wish
            to montage.
          </p>
          <label
            class="hover:bg-blue-600 bg-blue-500 rounded-md outline-none border-none text-white px-4 py-2"
            for="folder-uploader"
            >Select File
            <input
              id="folder-uploader"
              class="hidden"
              type="file"
              webkitdirectory
              @change="onFolderChange"
          /></label>
        </div>
        <div v-if="uploadMethod === 'image'" class="w-fit">
          <label
            class="hover:bg-blue-600 bg-blue-500 rounded-md outline-none border-none text-white px-4 py-2"
            for="image-uploader"
            >Add Image
            <input
              id="image-uploader"
              class="hidden"
              type="file"
              webkitdirectory
              @change="onImageUpload"
          /></label>
        </div>
      </div>
      <div v-if="selectedImages.length">
        <h2>Selected Images</h2>
        <span v-for="image of selectedImages">{{ image }}</span>
      </div>
      <div v-if="selectedFolder">
        <h2>Selected Folder</h2>
        <span>{{ selectedFolder }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
