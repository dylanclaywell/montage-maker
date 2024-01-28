<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'

import InputBox from './components/InputBox.vue'
import { FILTERS } from './lib/filter'

const selectedImages = ref<string[]>([])
const isTransparent = ref(false)

const frameWidth = ref('')
const frameHeight = ref('')
const framesPerRow = ref('')
const framesPerColumn = ref('')
const filter = ref('Catrom')
const outputDirectory = ref('')
const outputFilename = ref('')

function reset() {
  selectedImages.value = []
  isTransparent.value = false
  frameWidth.value = ''
  frameHeight.value = ''
  framesPerRow.value = ''
  framesPerColumn.value = ''
  filter.value = 'Catrom'
  outputDirectory.value = ''
  outputFilename.value = ''
}

async function openInputFileDialog() {
  const result = await open({
    multiple: true,
  })

  if (result === null) return

  selectedImages.value.push(...(Array.isArray(result) ? result : [result]))
}

async function openOutputFileDialog() {
  const result = await open({
    directory: true,
    multiple: false,
  })

  if (result === null) return

  if (Array.isArray(result)) {
    alert('Please select a single directory')
    return
  }

  outputDirectory.value = result
}

function numeric(value: string) {
  return /^\d+$/.test(value)
}

async function montage() {
  const options = {
    selectedImages: selectedImages.value,
    isTransparent: isTransparent.value,
    frameWidth: parseInt(frameWidth.value),
    frameHeight: parseInt(frameHeight.value),
    framesPerRow: parseInt(framesPerRow.value),
    framesPerColumn: parseInt(framesPerColumn.value),
    filter: filter.value,
    outputDirectory: outputDirectory.value,
    outputFilename: outputFilename.value,
  }

  const success = await invoke('montage', options)

  if (success) {
    alert(
      `Successfully combined images and saved to ${outputDirectory.value}/${outputFilename.value}`
    )
  } else {
    alert('Failed to combine images')
  }
}
</script>

<template>
  <main class="flex flex-col divide-y md:divide-x bg-gray-50">
    <section class="grid grid-cols-2 divide-y md:divide-x">
      <div class="p-4 flex flex-col gap-2 col-span-2 md:col-span-1">
        <h1 class="font-bold">Select Images</h1>
        <div class="flex flex-col gap-2">
          <button
            id="folder-uploader"
            class="hover:bg-blue-500 w-fit bg-blue-600 rounded-md outline-none border-none text-white px-4 py-2"
            webkitdirectory
            @click="openInputFileDialog"
          >
            Add Files
          </button>
        </div>
      </div>
      <div v-if="selectedImages.length" class="col-span-2 md:col-span-1">
        <div
          class="p-4 col-span-2 md:col-span-1 flex flex-col gap-2"
          v-if="selectedImages.length"
        >
          <h1 class="font-bold">Selected Images</h1>
          <div class="border">
            <span
              v-for="(image, index) of selectedImages"
              :class="{
                'p-1 w-full flex justify-between group break-all gap-2': true,
                'bg-gray-200': index % 2,
              }"
              >{{ image
              }}<button
                class="text-xs opacity-30 group-hover:opacity-100"
                @click="selectedImages.splice(index, 1)"
              >
                ❌
              </button></span
            >
          </div>
        </div>
      </div>
    </section>
    <section class="p-4 col-span-2 md:col-span-1">
      <h1 class="font-bold">Settings</h1>
      <div class="flex flex-col gap-2">
        <InputBox
          label="Frame Width"
          :validate="numeric"
          v-model="frameWidth"
        />
        <InputBox
          label="Frame Height"
          :validate="numeric"
          v-model="frameHeight"
        />
        <InputBox
          label="Frames Per Row"
          :validate="numeric"
          v-model="framesPerRow"
        />
        <InputBox
          label="Frames Per Column"
          :validate="numeric"
          v-model="framesPerColumn"
        />
        <label class="flex gap-2 items-center"
          ><input type="checkbox" v-model="isTransparent" /><span
            >Transparent</span
          ></label
        >
        <label class="flex flex-col">
          <span>Filter</span>
          <select v-model="filter">
            <option v-for="filter of FILTERS" :value="filter">
              {{ filter }}
            </option>
          </select>
        </label>
        <div class="flex items-end gap-2 w-full">
          <InputBox
            class="flex-grow"
            label="Output Directory"
            v-model="outputDirectory"
          />
          <button
            @click="openOutputFileDialog"
            class="cursor-default border border-gray-400 bg-gray-200 rounded-sm text-sm px-1 py-[0.1rem] hover:bg-gray-100"
          >
            Browse...
          </button>
        </div>
        <InputBox label="Output Filename" v-model="outputFilename" />
      </div>
    </section>
    <section
      class="flex flex-col items-center justify-between p-4 col-span-2 h-40"
    >
      <button
        class="w-fit hover:bg-blue-500 bg-blue-600 text-white px-4 py-2 rounded-md cursor-default"
        @click="montage"
      >
        Combine
      </button>
      <button
        @click="reset"
        class="text-blue-600 hover:text-blue-500 hover:underline"
      >
        Clear All
      </button>
    </section>
  </main>
</template>

<style scoped></style>
