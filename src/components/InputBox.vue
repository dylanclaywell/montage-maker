<script lang="ts" setup>
export type Props = {
  label: string
  modelValue: string
  validate?: (value: string) => boolean
}

export type Emits = {
  'update:modelValue': [string]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

function onInput(e: Event) {
  if (!(e.target instanceof HTMLInputElement)) return

  const target = e.target

  if (target.value && props.validate && !props.validate(e.target.value)) {
    target.value = props.modelValue
    return
  }

  const value = target.value
  emit('update:modelValue', value)
}
</script>

<template>
  <label class="flex flex-col">
    <span>{{ label }}</span>
    <input class="border rounded-md" :value="modelValue" @input="onInput"
  /></label>
</template>
