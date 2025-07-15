<template>
  <div>
    <h1 class="h1">Page Posts</h1>
    
    <!-- Hero Section -->
    <section class="bg-gradient-to-r from-purple-600 to-blue-600 py-16">
        <div class="container mx-auto px-4 text-center">
            <h2 class="text-4xl md:text-5xl font-bold text-white mb-4 animate-fade-in">
                Últimas Novidades
            </h2>
            <p class="text-xl text-purple-100 mb-8 animate-fade-in">
                Descubra as tendências mais quentes do e-commerce
            </p>
            <button class="bg-white text-purple-600 px-8 py-3 rounded-full font-semibold hover:bg-gray-100 transition-colors">
                Explorar Posts
            </button>
        </div>
    </section>

    <div v-if="pending">
        Loading ... <br>
        <span class="loading loading-spinner  size-32"></span>
      </div>

    <!-- Posts Grid -->
    <section v-else class="py-16">
        <div class="container mx-auto px-4">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                <!-- Post 1 -->
                <article v-for="post in posts" :key="post" class="bg-white rounded-2xl shadow-lg overflow-hidden card-hover animate-fade-in">
                    <div class="h-48 bg-gradient-to-br from-pink-400 to-purple-600 flex items-center justify-center">
                      <NuxtImg src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" loading="lazy" :alt="post?.title" />
                    </div>
                    <div class="p-6">
                        <div class="flex items-center space-x-2 mb-3">
                            <span class="bg-purple-100 text-purple-600 px-3 py-1 rounded-full text-sm font-medium">Tecnologia</span>
                            <span class="text-gray-500 text-sm">15 Jul 2025</span>
                        </div>
                        <h3 class="text-xl font-bold text-gray-800 mb-3">
                          {{ post?.title }}
                        </h3>
                        <p class="text-gray-600 mb-4">
                          {{ $truncate(post?.content, 70, '...') }}
                        </p>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-r from-purple-400 to-pink-400 rounded-full"></div>
                                <span class="text-sm text-gray-600">Por Maria Silva</span>
                            </div>
                            <NuxtLink :to="`/posts/${post.id}`" class="btn btn-primary">Read More</NuxtLink>
                        
                        </div>
                    </div>
                </article>

                <!-- Post 2 -->
                <article class="bg-white rounded-2xl shadow-lg overflow-hidden card-hover animate-fade-in">
                    <div class="h-48 bg-gradient-to-br from-blue-400 to-purple-600 flex items-center justify-center">
                        <i class="fas fa-chart-line text-white text-4xl"></i>
                    </div>
                    <div class="p-6">
                        <div class="flex items-center space-x-2 mb-3">
                            <span class="bg-blue-100 text-blue-600 px-3 py-1 rounded-full text-sm font-medium">Marketing</span>
                            <span class="text-gray-500 text-sm">14 Jul 2025</span>
                        </div>
                        <h3 class="text-xl font-bold text-gray-800 mb-3">
                            Estratégias de Marketing Digital para 2025
                        </h3>
                        <p class="text-gray-600 mb-4">
                            Conheça as principais tendências e estratégias de marketing digital que dominarão o mercado.
                        </p>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-r from-blue-400 to-purple-400 rounded-full"></div>
                                <span class="text-sm text-gray-600">Por João Santos</span>
                            </div>
                            <button class="text-purple-600 hover:text-purple-800 font-medium">
                                Ler mais →
                            </button>
                        </div>
                    </div>
                </article>

                <!-- Post 3 -->
                <article class="bg-white rounded-2xl shadow-lg overflow-hidden card-hover animate-fade-in">
                    <div class="h-48 bg-gradient-to-br from-green-400 to-blue-600 flex items-center justify-center">
                        <i class="fas fa-leaf text-white text-4xl"></i>
                    </div>
                    <div class="p-6">
                        <div class="flex items-center space-x-2 mb-3">
                            <span class="bg-green-100 text-green-600 px-3 py-1 rounded-full text-sm font-medium">Sustentabilidade</span>
                            <span class="text-gray-500 text-sm">13 Jul 2025</span>
                        </div>
                        <h3 class="text-xl font-bold text-gray-800 mb-3">
                            E-commerce Sustentável: Consumo Consciente
                        </h3>
                        <p class="text-gray-600 mb-4">
                            Como implementar práticas sustentáveis em seu negócio online e atrair consumidores conscientes.
                        </p>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-r from-green-400 to-blue-400 rounded-full"></div>
                                <span class="text-sm text-gray-600">Por Ana Costa</span>
                            </div>
                            <button class="text-purple-600 hover:text-purple-800 font-medium">
                                Ler mais →
                            </button>
                        </div>
                    </div>
                </article>

                <!-- Post 4 -->
                <article class="bg-white rounded-2xl shadow-lg overflow-hidden card-hover animate-fade-in">
                    <div class="h-48 bg-gradient-to-br from-orange-400 to-red-600 flex items-center justify-center">
                        <i class="fas fa-rocket text-white text-4xl"></i>
                    </div>
                    <div class="p-6">
                        <div class="flex items-center space-x-2 mb-3">
                            <span class="bg-orange-100 text-orange-600 px-3 py-1 rounded-full text-sm font-medium">Inovação</span>
                            <span class="text-gray-500 text-sm">12 Jul 2025</span>
                        </div>
                        <h3 class="text-xl font-bold text-gray-800 mb-3">
                            IA no E-commerce: Personalizando Experiências
                        </h3>
                        <p class="text-gray-600 mb-4">
                            Explore como a inteligência artificial está transformando a experiência do cliente online.
                        </p>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-r from-orange-400 to-red-400 rounded-full"></div>
                                <span class="text-sm text-gray-600">Por Pedro Lima</span>
                            </div>
                            <button class="text-purple-600 hover:text-purple-800 font-medium">
                                Ler mais →
                            </button>
                        </div>
                    </div>
                </article>

                <!-- Post 5 -->
                <article class="bg-white rounded-2xl shadow-lg overflow-hidden card-hover animate-fade-in">
                    <div class="h-48 bg-gradient-to-br from-indigo-400 to-purple-600 flex items-center justify-center">
                        <i class="fas fa-shield-alt text-white text-4xl"></i>
                    </div>
                    <div class="p-6">
                        <div class="flex items-center space-x-2 mb-3">
                            <span class="bg-indigo-100 text-indigo-600 px-3 py-1 rounded-full text-sm font-medium">Segurança</span>
                            <span class="text-gray-500 text-sm">11 Jul 2025</span>
                        </div>
                        <h3 class="text-xl font-bold text-gray-800 mb-3">
                            Segurança em Pagamentos Online: Melhores Práticas
                        </h3>
                        <p class="text-gray-600 mb-4">
                            Aprenda as melhores práticas para garantir a segurança dos pagamentos em sua loja online.
                        </p>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-r from-indigo-400 to-purple-400 rounded-full"></div>
                                <span class="text-sm text-gray-600">Por Carlos Oliveira</span>
                            </div>
                            <button class="text-purple-600 hover:text-purple-800 font-medium">
                                Ler mais →
                            </button>
                        </div>
                    </div>
                </article>

                <!-- Post 6 -->
                <article class="bg-white rounded-2xl shadow-lg overflow-hidden card-hover animate-fade-in">
                    <div class="h-48 bg-gradient-to-br from-pink-400 to-red-600 flex items-center justify-center">
                        <i class="fas fa-users text-white text-4xl"></i>
                    </div>
                    <div class="p-6">
                        <div class="flex items-center space-x-2 mb-3">
                            <span class="bg-pink-100 text-pink-600 px-3 py-1 rounded-full text-sm font-medium">Social</span>
                            <span class="text-gray-500 text-sm">10 Jul 2025</span>
                        </div>
                        <h3 class="text-xl font-bold text-gray-800 mb-3">
                            Social Commerce: Vendas Através das Redes Sociais
                        </h3>
                        <p class="text-gray-600 mb-4">
                            Descubra como aproveitar o poder das redes sociais para impulsionar suas vendas online.
                        </p>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-r from-pink-400 to-red-400 rounded-full"></div>
                                <span class="text-sm text-gray-600">Por Luciana Ferreira</span>
                            </div>
                            <button class="text-purple-600 hover:text-purple-800 font-medium">
                                Ler mais →
                            </button>
                        </div>
                    </div>
                </article>
            </div>

            <!-- Load More Button -->
            <div class="text-center mt-12">
                <button class="btn-primary text-white px-8 py-3 rounded-full font-semibold">
                    Carregar Mais Posts
                </button>
            </div>
        </div>
    </section>    

  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types';

const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();

const { pending, data: posts } = await useLazyFetch<Post>(`${config.public.baseURL}/api/posts`)
</script>

<style scoped>

@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css');
.gradient-bg {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }
        
        .card-hover {
            transition: all 0.3s ease;
        }
        
        .card-hover:hover {
            transform: translateY(-8px);
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
        }
        
        .animate-fade-in {
            animation: fadeIn 0.6s ease-out;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        
       
</style>
