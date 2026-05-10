<template>
  <div class="max-w-4xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ propsIsEditing ? 'Editar Produto' : 'Novo Produto' }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">Salvando produto...</span>
        </div>



        <!-- Reusable AppAlert component -->
        <AppAlert v-if="successMessage" type="success" :message="successMessage" :auto-close="3000" @close="successMessage = ''" />
        <AppAlert v-if="errorMessage" type="error" :message="errorMessage" :auto-close="5000" @close="errorMessage = ''" :dismissible="true" />

        <!-- Form (vee-validate) -->
        <form @submit.prevent="onSubmit" class="space-y-6" novalidate>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Nome do Produto *</span>
              </label>
              <input
                v-model="name"
                @blur="nameBlur"
                type="text"
                placeholder="Nome do produto"
                class="input input-bordered w-full"
                :class="{ 'input-error': nameError }"
                required
              />
              <label v-if="nameError" class="label">
                <span class="label-text-alt text-error">{{ nameError }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">SKU *</span>
              </label>
              <input
                v-model="sku"
                @blur="skuBlur"
                type="text"
                placeholder="SKU único"
                class="input input-bordered w-full"
                :class="{ 'input-error': skuError }"
                required
              />
              <label v-if="skuError" class="label">
                <span class="label-text-alt text-error">{{ skuError }}</span>
              </label>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Slug</span>
              </label>
              <input v-model="values.slug" type="text" placeholder="slug-do-produto" class="input input-bordered w-full" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Categoria</span>
              </label>
              <select v-model="values.categoryId" class="select select-bordered w-full">
                <option value="">Selecione uma categoria</option>
                <option v-for="category in categories" :key="category.id" :value="category.id">
                  {{ category.name }}
                </option>
              </select>
            </div>
          </div>

          <!-- Descriptions -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Descrição Curta</span>
            </label>
            <textarea v-model="values.shortDescription" placeholder="Descrição breve do produto" class="textarea textarea-bordered w-full" rows="3"></textarea>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Descrição Completa</span>
            </label>
            <textarea v-model="values.description" placeholder="Descrição detalhada do produto" class="textarea textarea-bordered w-full" rows="5"></textarea>
          </div>

          <!-- Pricing -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Preço de Venda *</span>
              </label>
              <input
                v-model="price"
                @blur="priceBlur"
                type="number"
                step="0.01"
                min="0"
                placeholder="0.00"
                class="input input-bordered w-full"
                :class="{ 'input-error': priceError }"
                required
              />
              <label v-if="priceError" class="label">
                <span class="label-text-alt text-error">{{ priceError }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Preço de Custo</span>
              </label>
              <input v-model="values.costPrice" type="number" step="0.01" min="0" placeholder="0.00" class="input input-bordered w-full" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Preço Comparativo</span>
              </label>
              <input v-model="values.comparePrice" type="number" step="0.01" min="0" placeholder="0.00" class="input input-bordered w-full" />
            </div>
          </div>

          <!-- Status and Features -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Status</span>
              </label>
              <select v-model="values.status" class="select select-bordered w-full">
                <option :value="1">Ativo</option>
                <option :value="0">Inativo</option>
                <option :value="2">Rascunho</option>
              </select>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">Produto em Destaque</span>
                <input v-model="values.featured" type="checkbox" class="checkbox checkbox-primary" />
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">Produto Ativo</span>
                <input v-model="values.active" type="checkbox" class="checkbox checkbox-primary" />
              </label>
            </div>
          </div>

          <!-- Image Upload Section -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Imagens do Produto</span>
            </label>

            <div class="flex items-center gap-4 mb-4">
              <input type="file" @change="handleImageUpload" multiple accept="image/*" class="file-input file-input-bordered w-full max-w-xs" :disabled="pending" />
              <button type="button" @click="addImageField()" class="btn btn-outline btn-sm" :disabled="pending">Adicionar Campo</button>
            </div>

            <div v-if="imageFields.length > 0" class="space-y-4">
              <div v-for="(field, index) in imageFields" :key="index" class="border rounded-lg p-4 bg-gray-50">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                  <div class="form-control">
                    <label class="label">
                      <span class="label-text">Imagem {{ index + 1 }}</span>
                    </label>
                    <input type="file" @change="(e) => handleImageFieldChange(e, index)" accept="image/*" class="file-input file-input-bordered w-full" :disabled="pending" />
                    <div v-if="field.preview" class="mt-2">
                      <img :src="field.preview" alt="Preview" class="w-20 h-20 object-cover rounded" />
                    </div>
                  </div>

                  <div class="form-control">
                    <label class="label">
                      <span class="label-text">Texto Alternativo</span>
                    </label>
                    <input v-model="field.alt_text" type="text" placeholder="Descrição da imagem" class="input input-bordered w-full" :disabled="pending" />
                  </div>

                  <div class="form-control">
                    <label class="label">
                      <span class="label-text">Posição</span>
                    </label>
                    <input v-model="field.position" type="number" min="0" class="input input-bordered w-full" :disabled="pending" />
                  </div>

                  <div class="form-control space-y-2">
                    <label class="label cursor-pointer">
                      <span class="label-text">Ativa</span>
                      <input v-model="field.active" type="checkbox" class="checkbox checkbox-primary" :disabled="pending" />
                    </label>
                    <label class="label cursor-pointer">
                      <span class="label-text">Capa</span>
                      <input v-model="field.cover" type="checkbox" class="checkbox checkbox-primary" :disabled="pending" @change="() => handleCoverChange(index)" />
                    </label>
                  </div>
                </div>

                <div class="flex justify-end mt-3">
                  <button type="button" @click="removeImageField(index)" class="btn btn-error btn-sm" :disabled="pending">Remover</button>
                </div>
              </div>
            </div>

            <div v-if="imageFields.length === 0" @drop.prevent="handleDrop" @dragover.prevent @dragenter.prevent class="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-primary transition-colors">
              <div class="text-gray-500">
                <svg class="mx-auto h-12 w-12 mb-4" stroke="currentColor" fill="none" viewBox="0 0 48 48">
                  <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                <p class="text-lg font-medium">Arraste e solte imagens aqui</p>
                <p class="text-sm">ou clique para selecionar arquivos</p>
              </div>
            </div>
          </div>

          <!-- Action Buttons -->
          <div class="flex justify-end space-x-4 pt-6">
            <button type="button" @click="emit('cancel')" class="btn btn-outline" :disabled="pending">Cancelar</button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ propsIsEditing ? 'Atualizar' : 'Salvar' }} Produto
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useForm, useField } from 'vee-validate'
import type { ProductApi, Category } from '~/types'

/* Props / Emits */
interface Props {
  product?: Partial<ProductApi>
  isEditing?: boolean
}
const props = withDefaults(defineProps<Props>(), { isEditing: false })
const emit = defineEmits<{
  (e: 'saved', product: ProductApi): void
  (e: 'cancel'): void
}>()

const config = useRuntimeConfig()
const propsIsEditing = computed(() => !!props.isEditing)

/* vee-validate form setup */
const { handleSubmit, values, validate, setFieldValue, errors } = useForm({
  initialValues: {
    name: '',
    slug: '',
    sku: '',
    shortDescription: '',
    description: '',
    price: 0,
    costPrice: 0,
    comparePrice: 0,
    featured: false,
    active: true,
    status: 1,
    categoryId: undefined
  }
})

/* useField for fields we want explicit errors / blur handling */
const { value: name, errorMessage: nameError, handleBlur: nameBlur } = useField('name', 'required')
const { value: sku, errorMessage: skuError, handleBlur: skuBlur } = useField('sku', 'required')
const { value: price, errorMessage: priceError, handleBlur: priceBlur } = useField('price', 'required|numeric|min_value:0.01')


/* state */
const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

/* image fields (unchanged logic) */
const imageFields = ref<Array<{
  file?: File | undefined
  preview: string
  alt_text: string
  position: number
  active: boolean
  cover: boolean
}>>([])

/* categories */
const { data: categoriesData } = useLazyFetch<Category[]>(`${config.public.baseURL}/api/categories`)
const categories = computed(() => categoriesData.value || [])

/* image handlers (unchanged) */
const addImageField = () => {
  const newField = {
    file: undefined,
    preview: '',
    alt_text: '',
    position: imageFields.value.length,
    active: true,
    cover: imageFields.value.length === 0
  }
  if (newField.cover) imageFields.value.forEach(f => (f.cover = false))
  imageFields.value.push(newField)
}

const handleImageUpload = (event: Event) => {
  const files = (event.target as HTMLInputElement).files
  if (files && files.length > 0) {
    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      if (file) {
        const reader = new FileReader()
        reader.onload = (e) => {
          const previewUrl = e.target?.result as string
          const newField = {
            file,
            preview: previewUrl,
            alt_text: file.name,
            position: imageFields.value.length,
            active: true,
            cover: imageFields.value.length === 0
          }
          if (newField.cover) imageFields.value.forEach(f => (f.cover = false))
          imageFields.value.push(newField)
        }
        reader.onerror = () => {
          console.error('Erro ao ler arquivo:', file.name)
        }
        reader.readAsDataURL(file)
      }
    }
  }
}

const handleImageFieldChange = (event: Event, index: number) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = (e) => {
      const previewUrl = e.target?.result as string
      imageFields.value[index].file = file
      imageFields.value[index].preview = previewUrl
      imageFields.value[index].alt_text = file.name
    }
    reader.onerror = () => {
      console.error('Erro ao ler arquivo:', file.name)
    }
    reader.readAsDataURL(file)
  }
}

const handleCoverChange = (index: number) => {
  if (imageFields.value[index].cover) {
    imageFields.value.forEach((f, i) => { if (i !== index) f.cover = false })
  }
}

const removeImageField = (index: number) => {
  const field = imageFields.value[index]
  // revokeObjectURL só deve ser usado para blob: URLs, não para data: URLs
  if (field.preview && field.preview.startsWith('blob:')) {
    URL.revokeObjectURL(field.preview)
  }
  imageFields.value.splice(index, 1)
  imageFields.value.forEach((f, i) => (f.position = i))
  if (imageFields.value.length && imageFields.value.every(f => !f.cover)) imageFields.value[0].cover = true
}

const handleDrop = (event: DragEvent) => {
  event.preventDefault()
  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      if (file && file.type.startsWith('image/')) {
        const reader = new FileReader()
        reader.onload = (e) => {
          const previewUrl = e.target?.result as string
          const newField = {
            file,
            preview: previewUrl,
            alt_text: file.name,
            position: imageFields.value.length,
            active: true,
            cover: imageFields.value.length === 0
          }
          if (newField.cover) imageFields.value.forEach(f => (f.cover = false))
          imageFields.value.push(newField)
        }
        reader.onerror = () => {
          console.error('Erro ao ler arquivo:', file.name)
        }
        reader.readAsDataURL(file)
      }
    }
  }
}

/* submit using vee-validate handler */
interface FormValues {
  name: string
  slug: string
  sku: string
  shortDescription: string
  description: string
  price: number
  costPrice: number
  comparePrice: number
  featured: boolean
  active: boolean
  status: number
  categoryId?: number
}

const onSubmit = handleSubmit(async (vals: FormValues) => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const url = props.isEditing ? `${config.public.baseURL}/api/products/${props.product?.id}` : `${config.public.baseURL}/api/products`
    const method = props.isEditing ? 'PUT' : 'POST'

    if (method === 'POST' && imageFields.value.some(f => f.file)) {
      const formData = new FormData()
      formData.append('name', vals.name || '')
      formData.append('slug', vals.slug || '')
      formData.append('sku', vals.sku || '')
      formData.append('short_description', vals.shortDescription || '')
      formData.append('description', vals.description || '')
      formData.append('price', (vals.price || 0).toString())
      formData.append('cost_price', (vals.costPrice || 0).toString())
      formData.append('compare_price', (vals.comparePrice || 0).toString())
      formData.append('featured', (vals.featured || false).toString())
      formData.append('active', (vals.active || true).toString())
      formData.append('status', (vals.status || 1).toString())
      formData.append('category_id', (vals.categoryId || 0).toString())

      imageFields.value.forEach((field, index) => {
        if (field.file) formData.append(`image${index}`, field.file)
        // optionally append alt_text, position, cover flags as form fields
        formData.append(`images[${index}][alt_text]`, field.alt_text || '')
        formData.append(`images[${index}][position]`, String(field.position))
        formData.append(`images[${index}][active]`, String(field.active))
        formData.append(`images[${index}][cover]`, String(field.cover))
      })

      const response = await $fetch(url, { method, body: formData })
      successMessage.value = 'Produto criado com sucesso!'
      emit('saved', response as ProductApi)
      resetForm()
    } else {
      // JSON path
      const payload = {
        ...vals,
        price: Number(vals.price) || 0,
        costPrice: Number(vals.costPrice) || 0,
        comparePrice: Number(vals.comparePrice) || 0,
        status: Number(vals.status) || 1,
        categoryId: Number(vals.categoryId) || undefined,
        featured: !!vals.featured,
        active: !!vals.active
      }
      const response = await $fetch(url, { method, body: payload })
      successMessage.value = props.isEditing ? 'Produto atualizado com sucesso!' : 'Produto criado com sucesso!'
      emit('saved', response as ProductApi)
      if (!props.isEditing) resetForm()
    }
    // Auto-close é gerenciado pelo componente AppAlert
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || 'Erro ao salvar produto. Tente novamente.'
  } finally {
    pending.value = false
  }
})

const resetForm = () => {
  setFieldValue('name', '')
  setFieldValue('slug', '')
  setFieldValue('sku', '')
  setFieldValue('shortDescription', '')
  setFieldValue('description', '')
  setFieldValue('price', 0)
  setFieldValue('costPrice', 0)
  setFieldValue('comparePrice', 0)
  setFieldValue('featured', false)
  setFieldValue('active', true)
  setFieldValue('status', 1)
  setFieldValue('categoryId', undefined)
  // Limpar blob URLs se existirem
  imageFields.value.forEach(field => {
    if (field.preview && field.preview.startsWith('blob:')) {
      URL.revokeObjectURL(field.preview)
    }
  })
  imageFields.value = []
}

/* populate values when editing */
onMounted(() => {
  if (props.product && props.isEditing) {
    const p = props.product
    setFieldValue('name', p.name || '')
    setFieldValue('slug', p.slug || '')
    setFieldValue('sku', p.sku || '')
    setFieldValue('shortDescription', p.shortDescription || '')
    setFieldValue('description', p.description || '')
    setFieldValue('price', p.price || 0)
    setFieldValue('costPrice', p.costPrice || 0)
    setFieldValue('comparePrice', p.comparePrice || 0)
    setFieldValue('featured', !!p.featured)
    setFieldValue('active', p.active ?? true)
    setFieldValue('status', p.status ?? 1)
    setFieldValue('categoryId', p.categoryId)

    if (p.images && p.images.length) {
      p.images.forEach((image, index) => {
        imageFields.value.push({
          file: undefined,
          preview: image.image ? `${config.public.baseURL}/uploads/products/${image.image}` : '',
          alt_text: image.alt_text || '',
          position: image.position ?? index,
          active: image.active ?? true,
          cover: image.cover ?? false
        })
      })
    }
  }
})
</script>

<style scoped></style>