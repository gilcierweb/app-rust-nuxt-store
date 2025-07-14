<template>
    <div>
        <div v-if="pending">
            Loading ... <br>
            <span class="loading loading-spinner  size-32"></span>
        </div>
        
        <!-- Topo Colorido da Categoria -->
        <section v-else class="relative bg-gradient-to-br from-primary via-secondary to-accent overflow-hidden">
            <div class="absolute inset-0 bg-black/20"></div>
            <div class="absolute inset-0 bg-gradient-to-r from-primary/90 to-transparent"></div>

            <!-- Elementos decorativos -->
            <div
                class="absolute top-0 right-0 w-96 h-96 bg-white/10 rounded-full -translate-y-20 translate-x-20 blur-3xl">
            </div>
            <div
                class="absolute bottom-0 left-0 w-72 h-72 bg-white/5 rounded-full translate-y-20 -translate-x-20 blur-2xl">
            </div>

            <!-- Ícones flutuantes -->
            <div class="absolute top-10 right-20 text-white/20 text-4xl">
                <i class="fas fa-laptop"></i>
            </div>
            <div class="absolute bottom-10 left-20 text-white/20 text-3xl">
                <i class="fas fa-mobile-alt"></i>
            </div>
            <div class="absolute top-20 left-1/3 text-white/20 text-2xl">
                <i class="fas fa-headphones"></i>
            </div>

            <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <div class="text-center">
                    <div
                        class="inline-flex items-center justify-center w-20 h-20 bg-white/20 backdrop-blur-sm rounded-2xl mb-6">
                        <i class="fas fa-laptop text-3xl text-white"></i>
                    </div>

                    <h1 class="text-4xl md:text-5xl font-bold text-white mb-4">
                        {{ category?.name }}
                    </h1>

                    <p class="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
                        {{ category.active }} - {{ category.position }} <br>
                        {{ $truncate(category?.description, 70, '...') }}
                    </p>

                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center mb-8">
                        <div class="flex items-center text-white/80">
                            <i class="fas fa-box-open mr-2"></i>
                            <span>2.847 produtos</span>
                        </div>
                        <div class="hidden sm:block w-1 h-1 bg-white/40 rounded-full"></div>
                        <div class="flex items-center text-white/80">
                            <i class="fas fa-star mr-2"></i>
                            <span>4.6/5 avaliação média</span>
                        </div>
                        <div class="hidden sm:block w-1 h-1 bg-white/40 rounded-full"></div>
                        <div class="flex items-center text-white/80">
                            <i class="fas fa-shipping-fast mr-2"></i>
                            <span>Entrega rápida</span>
                        </div>
                    </div>

                    <!-- Promoção Especial -->
                    <div class="bg-white/20 backdrop-blur-sm rounded-2xl p-6 max-w-md mx-auto">
                        <div class="flex items-center justify-center mb-3">
                            <i class="fas fa-bolt text-yellow-300 mr-2"></i>
                            <span class="text-white font-semibold">Oferta Relâmpago</span>
                        </div>
                        <p class="text-white/90 text-sm mb-4">
                            Até 70% OFF em produtos selecionados
                        </p>
                        <button class="btn btn-warning btn-sm w-full">
                            <i class="fas fa-clock mr-2"></i>
                            Ver Ofertas
                        </button>
                    </div>
                </div>
            </div>
        </section>


        <!-- Filters Section -->
        <section class="bg-white shadow-lg sticky top-0 z-20">
            <div class="max-w-7xl mx-auto px-6 py-6">
                <div class="flex flex-wrap items-center justify-between gap-4">
                    <div class="flex items-center space-x-4">
                        <span class="text-gray-600 font-medium">Filtros:</span>
                        <select
                            class="border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-purple-500">
                            <option>Todas as marcas</option>
                            <option>Apple</option>
                            <option>Samsung</option>
                            <option>Dell</option>
                            <option>HP</option>
                        </select>
                        <select
                            class="border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-purple-500">
                            <option>Preço</option>
                            <option>Menor preço</option>
                            <option>Maior preço</option>
                        </select>
                        <select
                            class="border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-purple-500">
                            <option>Avaliação</option>
                            <option>5 estrelas</option>
                            <option>4+ estrelas</option>
                            <option>3+ estrelas</option>
                        </select>
                    </div>
                    <div class="flex items-center space-x-4">
                        <span class="text-gray-600">2,847 produtos encontrados</span>
                        <div class="flex items-center space-x-2">
                            <button class="p-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                                <i class="fas fa-th-large"></i>
                            </button>
                            <button class="p-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                                <i class="fas fa-list"></i>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- Products Grid -->
        <section class="py-16">
            <div class="max-w-7xl mx-auto px-6">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
                    <!-- Product Card 1 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1546868871-7041f2a55e12?w=400&h=300&fit=crop"
                                alt="iPhone 15" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 right-4 bg-red-500 text-white px-2 py-1 rounded-lg text-sm font-medium">
                                -15%
                            </div>
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">iPhone 15 Pro Max</h3>
                            <p class="text-gray-600 text-sm mb-4">Apple • 256GB • Titânio Natural</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(234 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 8.999</span>
                                    <span class="text-gray-500 line-through text-sm ml-2">R$ 10.599</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 2 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1593642702821-c8da6771f0c6?w=400&h=300&fit=crop"
                                alt="MacBook Pro" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">MacBook Pro 14"</h3>
                            <p class="text-gray-600 text-sm mb-4">Apple • M3 Pro • 18GB RAM • 512GB SSD</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(187 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 16.999</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 3 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1544244015-0df4b3ffc6b0?w=400&h=300&fit=crop"
                                alt="Galaxy S24" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 right-4 bg-green-500 text-white px-2 py-1 rounded-lg text-sm font-medium">
                                Novo
                            </div>
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">Galaxy S24 Ultra</h3>
                            <p class="text-gray-600 text-sm mb-4">Samsung • 512GB • Preto Titânio</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(156 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 7.299</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 4 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1541807084-5c52b6b3adef?w=400&h=300&fit=crop"
                                alt="iPad Pro" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 right-4 bg-red-500 text-white px-2 py-1 rounded-lg text-sm font-medium">
                                -20%
                            </div>
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">iPad Pro 12.9"</h3>
                            <p class="text-gray-600 text-sm mb-4">Apple • M2 • 256GB • Space Gray</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(298 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 8.799</span>
                                    <span class="text-gray-500 line-through text-sm ml-2">R$ 10.999</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 5 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1525547719571-a2d4ac8945e2?w=400&h=300&fit=crop"
                                alt="Dell XPS" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">Dell XPS 13</h3>
                            <p class="text-gray-600 text-sm mb-4">Dell • Intel i7 • 16GB RAM • 512GB SSD</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="far fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(143 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 5.999</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 6 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1572569511254-d8f925fe2cbb?w=400&h=300&fit=crop"
                                alt="AirPods Pro" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 right-4 bg-blue-500 text-white px-2 py-1 rounded-lg text-sm font-medium">
                                Mais vendido
                            </div>
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">AirPods Pro 2ª Gen</h3>
                            <p class="text-gray-600 text-sm mb-4">Apple • Cancelamento de ruído • USB-C</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(567 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 1.899</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 7 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1542744173-8e7e53415bb0?w=400&h=300&fit=crop"
                                alt="Surface Laptop" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">Surface Laptop 5</h3>
                            <p class="text-gray-600 text-sm mb-4">Microsoft • Intel i7 • 16GB RAM • 512GB SSD</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="far fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(89 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 7.499</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Product Card 8 -->
                    <div class="product-card rounded-2xl overflow-hidden card-hover">
                        <div class="relative">
                            <img src="https://images.unsplash.com/photo-1583394838336-acd977736f90?w=400&h=300&fit=crop"
                                alt="Apple Watch" class="w-full h-48 object-cover">
                            <div
                                class="absolute top-4 right-4 bg-red-500 text-white px-2 py-1 rounded-lg text-sm font-medium">
                                -10%
                            </div>
                            <div
                                class="absolute top-4 left-4 bg-white/90 backdrop-blur-sm rounded-full p-2 hover:bg-white transition-colors cursor-pointer">
                                <i class="fas fa-heart text-gray-600"></i>
                            </div>
                        </div>
                        <div class="p-6">
                            <h3 class="font-semibold text-lg mb-2">Apple Watch Series 9</h3>
                            <p class="text-gray-600 text-sm mb-4">Apple • 45mm • GPS + Cellular • Azul</p>
                            <div class="flex items-center mb-4">
                                <div class="flex text-yellow-400">
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                    <i class="fas fa-star"></i>
                                </div>
                                <span class="text-gray-600 text-sm ml-2">(412 avaliações)</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-2xl font-bold text-gray-900">R$ 3.599</span>
                                    <span class="text-gray-500 line-through text-sm ml-2">R$ 3.999</span>
                                </div>
                                <button
                                    class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition-colors">
                                    <i class="fas fa-shopping-cart"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Load More Button -->
                <div class="text-center mt-16">
                    <button
                        class="bg-purple-600 text-white font-semibold px-8 py-4 rounded-xl hover:bg-purple-700 transition-colors shadow-lg hover:shadow-xl transform hover:scale-105 transition-all duration-300">
                        Carregar mais produtos
                        <i class="fas fa-arrow-down ml-2"></i>
                    </button>
                </div>
            </div>
        </section>




        <!-- Breadcrumb -->
        <div class="breadcrumbs">
            <ul>
                <li>
                    <a href="#">Home</a>
                </li>
                <li class="breadcrumbs-separator rtl:rotate-180"><span class="icon-[tabler--chevron-right]"></span></li>
                <li>
                    <a href="#">Categorias</a>
                </li>
                <li class="breadcrumbs-separator rtl:rotate-180"><span class="icon-[tabler--chevron-right]"></span></li>
                <li aria-current="page">Eletrônicos</li>
            </ul>
        </div>

        <!-- Conteúdo Principal -->
        <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <!-- Categorias em Destaque -->
            <div class="mb-8">
                <h2 class="text-2xl font-bold text-gray-900 mb-6">Categorias em Destaque</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
                    <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-4 text-center">
                        <i class="fas fa-mobile-alt text-3xl text-primary mb-2"></i>
                        <span class="text-sm font-medium">Smartphones</span>
                    </div>
                    <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-4 text-center">
                        <i class="fas fa-laptop text-3xl text-secondary mb-2"></i>
                        <span class="text-sm font-medium">Notebooks</span>
                    </div>
                    <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-4 text-center">
                        <i class="fas fa-headphones text-3xl text-accent mb-2"></i>
                        <span class="text-sm font-medium">Headphones</span>
                    </div>
                    <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-4 text-center">
                        <i class="fas fa-camera text-3xl text-success mb-2"></i>
                        <span class="text-sm font-medium">Câmeras</span>
                    </div>
                    <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-4 text-center">
                        <i class="fas fa-gamepad text-3xl text-warning mb-2"></i>
                        <span class="text-sm font-medium">Games</span>
                    </div>
                    <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-4 text-center">
                        <i class="fas fa-tv text-3xl text-error mb-2"></i>
                        <span class="text-sm font-medium">TVs</span>
                    </div>
                </div>
            </div>

            <!-- Grid de Produtos -->
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                <!-- Produto 1 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Smartphone" class="w-full h-48 object-cover">
                        <div class="absolute top-2 left-2">
                            <span class="badge badge-error text-white">-25%</span>
                        </div>
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Smart TV 55" 4K Ultra HD
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(234 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 1.799</span>
                                <span class="text-sm text-gray-500 line-through">R$ 2.599</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>
                <!-- Produto 2 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1496181133206-80ce9b88a853?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Notebook" class="w-full h-48 object-cover">
                        <div class="absolute top-2 left-2">
                            <span class="badge badge-warning text-white">Oferta</span>
                        </div>
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Notebook UltraBook Pro
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(89 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 2.899</span>
                                <span class="text-sm text-gray-500 line-through">R$ 3.499</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>

                <!-- Produto 3 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1505740420928-5e560c06d30e?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Headphones" class="w-full h-48 object-cover">
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Headphones Wireless Premium
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(256 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 599</span>
                            </div>
                            <span class="badge badge-warning">Últimas unidades</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>

                <!-- Produto 4 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1606983340126-99ab4feaa64a?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Camera" class="w-full h-48 object-cover">
                        <div class="absolute top-2 left-2">
                            <span class="badge badge-primary text-white">Novo</span>
                        </div>
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Câmera Digital 4K
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(47 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 1.299</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>

                <!-- Produto 5 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1593305841991-05c297ba4575?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Console" class="w-full h-48 object-cover">
                        <div class="absolute top-2 left-2">
                            <span class="badge badge-secondary text-white">Trending</span>
                        </div>
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Console Gaming Pro
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(312 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 2.499</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>

                <!-- Produto 7 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1572569511254-d8f925fe2cbb?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Tablet" class="w-full h-48 object-cover">
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Tablet Pro 12.9" 256GB
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(76 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 3.299</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>

                <!-- Produto 8 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1585792180666-f7347c490ee2?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Smartwatch" class="w-full h-48 object-cover">
                        <div class="absolute top-2 left-2">
                            <span class="badge badge-accent text-white">Bestseller</span>
                        </div>
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Smartwatch Fitness Pro
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(423 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 799</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>
                <!-- Produto 8 -->
                <div class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <figure class="relative">
                        <img src="https://images.unsplash.com/photo-1585792180666-f7347c490ee2?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=500&q=80"
                            alt="Smartwatch" class="w-full h-48 object-cover">
                        <div class="absolute top-2 left-2">
                            <span class="badge badge-accent text-white">Bestseller</span>
                        </div>
                        <div class="absolute top-2 right-2">
                            <button class="btn btn-ghost btn-sm btn-circle text-gray-400 hover:text-red-500">
                                <i class="fas fa-heart"></i>
                            </button>
                        </div>
                    </figure>
                    <div class="card-body p-4">
                        <h3 class="card-title text-lg font-semibold mb-2">
                            Smartwatch Fitness Pro
                        </h3>
                        <div class="flex items-center mb-4">
                            <div class="flex text-yellow-400">
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                                <i class="fas fa-star"></i>
                            </div>
                            <span class="text-gray-600 text-sm ml-2">(34 avaliações)</span>
                        </div>
                        <div class="flex items-center justify-between mb-3">
                            <div class="flex items-center space-x-2">
                                <span class="text-lg font-bold text-gray-900">R$ 799</span>
                            </div>
                            <span class="badge badge-success">Em estoque</span>
                        </div>
                        <button class="btn btn-primary w-full">
                            <i class="fas fa-cart-plus mr-2"></i>
                            Adicionar ao Carrinho
                        </button>
                    </div>
                </div>
            </div>

            <!-- Banner de Promoção -->
            <div
                class="bg-gradient-to-r from-primary to-secondary rounded-2xl p-8 my-12 text-white relative overflow-hidden">
                <div class="absolute top-0 right-0 w-32 h-32 bg-white/10 rounded-full -translate-y-8 translate-x-8">
                </div>
                <div class="absolute bottom-0 left-0 w-24 h-24 bg-white/10 rounded-full translate-y-8 -translate-x-8">
                </div>
                <div class="relative z-10 text-center">
                    <h3 class="text-3xl font-bold mb-4">Oferta Especial do Dia!</h3>
                    <p class="text-lg mb-6 opacity-90">Até 50% OFF em produtos selecionados + Frete Grátis</p>
                    <button class="btn btn-warning btn-lg">
                        <i class="fas fa-bolt mr-2"></i>
                        Ver Ofertas
                    </button>
                </div>
            </div>

            <!-- Paginação -->
            <div class="flex justify-center mt-12">
                <div class="join">
                    <button class="join-item btn btn-outline">
                        <i class="fas fa-chevron-left"></i>
                    </button>
                    <button class="join-item btn btn-outline">1</button>
                    <button class="join-item btn btn-primary">2</button>
                    <button class="join-item btn btn-outline">3</button>
                    <button class="join-item btn btn-outline">4</button>
                    <button class="join-item btn btn-outline">5</button>
                    <button class="join-item btn btn-outline">...</button>
                    <button class="join-item btn btn-outline">24</button>
                    <button class="join-item btn btn-outline">
                        <i class="fas fa-chevron-right"></i>
                    </button>
                </div>
            </div>
        </main>

    </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types';

const route = useRoute()
const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();

const id = route.params.id;

const { pending, data: category } = await useLazyFetch<Category>(`${config.public.baseURL}/api/categories/${id}`);

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
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

.floating-animation {
    animation: float 6s ease-in-out infinite;
}

@keyframes float {

    0%,
    100% {
        transform: translateY(0px);
    }

    50% {
        transform: translateY(-20px);
    }
}

.stats-card {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
}

.product-card {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.animate-pulse-slow {
    animation: pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>