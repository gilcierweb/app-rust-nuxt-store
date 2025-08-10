<template>
    <div class="max-w-4xl mx-auto">
        <div class="card bg-white shadow-lg">
            <div class="card-body">
                <h2 class="card-title text-2xl font-bold mb-6">
                    {{ isEditing ? 'Editar Produto' : 'Novo Produto' }}
                </h2>

                <!-- Loading State -->
                <div v-if="pending" class="flex items-center justify-center py-8">
                    <span class="loading loading-spinner text-primary size-12"></span>
                    <span class="ml-3">Salvando produto...</span>
                </div>

                <!-- Success Message -->
                <div v-if="successMessage" class="alert alert-success mb-6 flex items-center gap-4" role="alert">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none"
                        viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span>{{ successMessage }}</span>
                </div>

                <!-- Error Message -->
                <div v-if="errorMessage" class="alert alert-error mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none"
                        viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span>{{ errorMessage }}</span>
                </div>

                <form @submit.prevent="handleSubmit" class="space-y-6">
                    <!-- Basic Information -->
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Nome do Produto *</span>
                            </label>
                            <input v-model="form.name" type="text" placeholder="Nome do produto"
                                class="input input-bordered w-full" :class="{ 'input-error': errors.name }" required />
                            <label v-if="errors.name" class="label">
                                <span class="label-text-alt text-error">{{ errors.name }}</span>
                            </label>
                        </div>

                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">SKU *</span>
                            </label>
                            <input v-model="form.sku" type="text" placeholder="SKU único"
                                class="input input-bordered w-full" :class="{ 'input-error': errors.sku }" required />
                            <label v-if="errors.sku" class="label">
                                <span class="label-text-alt text-error">{{ errors.sku }}</span>
                            </label>
                        </div>
                    </div>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text font-semibold">Slug</span>
                        </label>
                        <input v-model="form.slug" type="text" placeholder="slug-do-produto"
                            class="input input-bordered w-full" :class="{ 'input-error': errors.slug }" />
                        <label v-if="errors.slug" class="label">
                            <span class="label-text-alt text-error">{{ errors.slug }}</span>
                        </label>
                    </div>

                    <!-- Descriptions -->
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Descrição Curta</span>
                            </label>
                            <textarea v-model="form.shortDescription" placeholder="Descrição breve do produto"
                                class="textarea textarea-bordered w-full" rows="3"></textarea>
                        </div>

                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Categoria</span>
                            </label>
                            <select v-model="form.categoryId" class="select select-bordered w-full">
                                <option value="">Selecione uma categoria</option>
                                <option v-for="category in categories" :key="category.id" :value="category.id">
                                    {{ category.name }}
                                </option>
                            </select>
                        </div>
                    </div>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text font-semibold">Descrição Completa</span>
                        </label>
                        <textarea v-model="form.description" placeholder="Descrição detalhada do produto"
                            class="textarea textarea-bordered w-full" rows="5"></textarea>
                    </div>

                    <!-- Pricing -->
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Preço de Venda *</span>
                            </label>
                            <input v-model="form.price" type="number" step="0.01" min="0" placeholder="0.00"
                                class="input input-bordered w-full" :class="{ 'input-error': errors.price }" required />
                            <label v-if="errors.price" class="label">
                                <span class="label-text-alt text-error">{{ errors.price }}</span>
                            </label>
                        </div>

                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Preço de Custo</span>
                            </label>
                            <input v-model="form.costPrice" type="number" step="0.01" min="0" placeholder="0.00"
                                class="input input-bordered w-full" />
                        </div>

                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Preço Comparativo</span>
                            </label>
                            <input v-model="form.comparePrice" type="number" step="0.01" min="0" placeholder="0.00"
                                class="input input-bordered w-full" />
                        </div>
                    </div>

                    <!-- Status and Features -->
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text font-semibold">Status</span>
                            </label>
                            <select v-model="form.status" class="select select-bordered w-full">
                                <option :value="1">Ativo</option>
                                <option :value="0">Inativo</option>
                                <option :value="2">Rascunho</option>
                            </select>
                        </div>

                        <div class="form-control">
                            <label class="label cursor-pointer">
                                <span class="label-text font-semibold">Produto em Destaque</span>
                                <input v-model="form.featured" type="checkbox" class="checkbox checkbox-primary" />
                            </label>
                        </div>

                        <div class="form-control">
                            <label class="label cursor-pointer">
                                <span class="label-text font-semibold">Produto Ativo</span>
                                <input v-model="form.active" type="checkbox" class="checkbox checkbox-primary" />
                            </label>
                        </div>
                    </div>

                    <!-- Action Buttons -->
                    <div class="flex justify-end space-x-4 pt-6">
                        <button type="button" @click="$emit('cancel')" class="btn btn-outline" :disabled="pending">
                            Cancelar
                        </button>
                        <button type="submit" class="btn btn-primary" :disabled="pending">
                            <span v-if="pending" class="loading loading-spinner loading-sm"></span>
                            {{ isEditing ? 'Atualizar' : 'Salvar' }} Produto
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import type { ProductApi, Category } from '~/types'

interface Props {
    product?: Partial<ProductApi>
    isEditing?: boolean
}

interface Emits {
    (e: 'saved', product: ProductApi): void
    (e: 'cancel'): void
}

const props = withDefaults(defineProps<Props>(), {
    isEditing: false
})

const emit = defineEmits<Emits>()
const config = useRuntimeConfig()

const form = ref<Partial<ProductApi>>({
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
    categoryId: undefined,
    ...props.product
})

const errors = ref<Record<string, string>>({})
const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Fetch categories for dropdown
const { data: categoriesData } = await useFetch<Category[]>(`${config.public.baseURL}/api/categories`)
const categories = computed(() => categoriesData.value || [])

// Auto-generate slug from name
watch(() => form.value.name, (newName) => {
    if (newName && !props.isEditing) {
        form.value.slug = newName
            .toLowerCase()
            .replace(/[^a-z0-9\s-]/g, '')
            .replace(/\s+/g, '-')
            .replace(/-+/g, '-')
            .trim()
    }
})

// Validation function
const validateForm = (): boolean => {
    errors.value = {}
    if (!form.value.name?.trim()) errors.value.name = 'Nome é obrigatório'
    if (!form.value.sku?.trim()) errors.value.sku = 'SKU é obrigatório'
    if (!form.value.price || parseFloat(form.value.price as string) <= 0) errors.value.price = 'Preço deve ser maior que zero'
    return Object.keys(errors.value).length === 0
}

// Handle form submission
const handleSubmit = async () => {
    if (!validateForm()) return

    pending.value = true
    errorMessage.value = ''
    successMessage.value = ''

    try {
        const url = props.isEditing
            ? `${config.public.baseURL}/api/products/${props.product?.id}`
            : `${config.public.baseURL}/api/products`
        const method = props.isEditing ? 'PUT' : 'POST'
       
        const payload: ProductApi = {
            ...form.value,
            price: form.value.price ? Number(form.value.price) : 0,
            cost_price: form.value.costPrice ? Number(form.value.costPrice) : 0,
            compare_price: form.value.comparePrice ? Number(form.value.comparePrice) : 0,
            status: Number(form.value.status),
            category_id: form.value.categoryId ? Number(form.value.categoryId) : undefined,
            featured: !!form.value.featured,
            active: !!form.value.active,
            name: form.value.name || '',
            slug: form.value.slug || '',
            sku: form.value.sku || '',
            short_description: form.value.shortDescription || '',
            description: form.value.description || '',
            id: props.product?.id || 0
        }

        const response = await $fetch(url, {
            method,
            body: payload,
        })

        successMessage.value = props.isEditing
            ? 'Produto atualizado com sucesso!'
            : 'Produto criado com sucesso!'

        emit('saved', response as ProductApi)

        if (!props.isEditing) resetForm()

        setTimeout(() => { successMessage.value = '' }, 3000)
    } catch (err: any) {
        errorMessage.value = err.message || 'Erro ao salvar produto. Tente novamente.'
        setTimeout(() => { errorMessage.value = '' }, 5000)
    } finally {
        pending.value = false
    }
}

// Reset form to initial state
const resetForm = () => {
    form.value = {
        name: '',
        slug: '',
        sku: '',
        shortDescription: '',
        description: '',
        price: '',
        costPrice: '',
        comparePrice: '',
        featured: false,
        active: true,
        status: 1,
        categoryId: undefined,
    }
    errors.value = {}
}

// Initialize form with product data if editing
onMounted(() => {
    if (props.product && props.isEditing) {
        form.value = { ...props.product }
    }
})
</script>

<style scoped></style>