<template>
    <div>
        <div v-if="pending">
            Loading ... <br />
            <span class="loading loading-spinner size-32"></span>
        </div>
         

    <!-- Seção do Topo com Fundo Colorido -->
    <section v-else class="gradient-bg py-16 relative overflow-hidden">
        <div class="absolute inset-0 bg-black opacity-10"></div>
        <div class="absolute top-10 left-10 animate-float">
            <div class="w-20 h-20 bg-white opacity-10 rounded-full"></div>
        </div>
        <div class="absolute bottom-10 right-10 animate-float" style="animation-delay: 1s;">
            <div class="w-16 h-16 bg-white opacity-10 rounded-full"></div>
        </div>
        <div class="absolute top-1/2 left-1/4 animate-float" style="animation-delay: 2s;">
            <div class="w-12 h-12 bg-white opacity-10 rounded-full"></div>
        </div>
        
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
            <div class="text-center">
                <div class="mb-8">
                    <div class="relative inline-block">
                        <img src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=200&h=200&q=80" 
                             alt="Foto do Perfil" 
                             class="w-32 h-32 rounded-full mx-auto border-4 border-white shadow-2xl">
                        <div class="absolute bottom-2 right-2 bg-green-500 w-6 h-6 rounded-full border-2 border-white">
 <button class="absolute bottom-0 right-0 bg-blue-500 hover:bg-blue-600 w-8 h-8 rounded-full border-2 border-white shadow-lg flex items-center justify-center text-white text-sm transition-colors">
                            <i class="fas fa-camera"></i>
                        </button>

                        </div>
                    </div>
                </div>
                
                <h1 class="text-4xl font-bold text-white mb-2">{{ profile?.full_name }}</h1>
                <p class="text-white text-lg opacity-90 mb-6">Cliente Premium desde 2020</p>
                
                <div class="flex justify-center space-x-4">
                    <div class="bg-white bg-opacity-20 backdrop-blur-md rounded-lg px-6 py-3">
                        <div class="text-2xl font-bold text-black">156</div>
                        <div class="text-black text-sm opacity-80">Pedidos</div>
                    </div>
                    <div class="bg-white bg-opacity-20 backdrop-blur-md rounded-lg px-6 py-3">
                        <div class="text-2xl font-bold text-black">R$ 25.680</div>
                        <div class="text-black text-sm opacity-80">Total Gasto</div>
                    </div>
                    <div class="bg-white bg-opacity-20 backdrop-blur-md rounded-lg px-6 py-3">
                        <div class="text-2xl font-bold text-black">4.8</div>
                        <div class="text-black text-sm opacity-80">Avaliação</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Conteúdo Principal -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <!-- Coluna Esquerda -->
            <div class="lg:col-span-2 space-y-6">
                <!-- Informações Pessoais -->
                <div class="profile-card rounded-xl p-6 shadow-lg hover-lift">
                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-2xl font-bold text-gray-900">Informações Pessoais</h2>
                        <button class="btn btn-primary btn-sm">
                            <i class="fas fa-edit mr-2"></i>
                            Editar
                        </button>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">Nome Completo</label>
                            <div class="input-group">
                                <span class="input-group-text"><i class="fas fa-user"></i></span>
                                <input type="text" class="input input-bordered w-full" value="João Silva" readonly>
                            </div>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">Email</label>
                            <div class="input-group">
                                <span class="input-group-text"><i class="fas fa-envelope"></i></span>
                                <input type="email" class="input input-bordered w-full" value="joao@exemplo.com" readonly>
                            </div>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">Telefone</label>
                            <div class="input-group">
                                <span class="input-group-text"><i class="fas fa-phone"></i></span>
                                <input type="tel" class="input input-bordered w-full" value="(11) 99999-9999" readonly>
                            </div>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">Data de Nascimento</label>
                            <div class="input-group">
                                <span class="input-group-text"><i class="fas fa-calendar"></i></span>
                                <input type="date" class="input input-bordered w-full" value="1990-05-15" readonly>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Últimos Pedidos -->
                <div class="profile-card rounded-xl p-6 shadow-lg hover-lift">
                    <h2 class="text-2xl font-bold text-gray-900 mb-6">Últimos Pedidos</h2>
                    
                    <div class="space-y-4">
                        <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                            <div class="flex items-center space-x-4">
                                <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
                                    <i class="fas fa-box text-blue-600"></i>
                                </div>
                                <div>
                                    <div class="font-semibold text-gray-900">Pedido #12345</div>
                                    <div class="text-sm text-gray-500">05 de Julho, 2025</div>
                                </div>
                            </div>
                            <div class="text-right">
                                <div class="font-semibold text-gray-900">R$ 299,90</div>
                                <div class="badge badge-success">Entregue</div>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                            <div class="flex items-center space-x-4">
                                <div class="w-12 h-12 bg-orange-100 rounded-lg flex items-center justify-center">
                                    <i class="fas fa-truck text-orange-600"></i>
                                </div>
                                <div>
                                    <div class="font-semibold text-gray-900">Pedido #12344</div>
                                    <div class="text-sm text-gray-500">03 de Julho, 2025</div>
                                </div>
                            </div>
                            <div class="text-right">
                                <div class="font-semibold text-gray-900">R$ 159,90</div>
                                <div class="badge badge-warning">Em Trânsito</div>
                            </div>
                        </div>
                        
                        <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                            <div class="flex items-center space-x-4">
                                <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                                    <i class="fas fa-clock text-purple-600"></i>
                                </div>
                                <div>
                                    <div class="font-semibold text-gray-900">Pedido #12343</div>
                                    <div class="text-sm text-gray-500">01 de Julho, 2025</div>
                                </div>
                            </div>
                            <div class="text-right">
                                <div class="font-semibold text-gray-900">R$ 89,90</div>
                                <div class="badge badge-info">Preparando</div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="mt-6 text-center">
                        <button class="btn btn-outline btn-primary">Ver Todos os Pedidos</button>
                    </div>
                </div>
            </div>

            <!-- Coluna Direita -->
            <div class="space-y-6">
                <!-- Cards de Estatísticas -->
                <div class="stats-card rounded-xl p-6 shadow-lg hover-lift text-white">
                    <div class="flex items-center justify-between">
                        <div>
                            <div class="text-2xl font-bold">156</div>
                            <div class="text-sm opacity-90">Total de Pedidos</div>
                        </div>
                        <div class="text-3xl opacity-80">
                            <i class="fas fa-shopping-bag"></i>
                        </div>
                    </div>
                </div>

                <div class="orders-card rounded-xl p-6 shadow-lg hover-lift text-white">
                    <div class="flex items-center justify-between">
                        <div>
                            <div class="text-2xl font-bold">23</div>
                            <div class="text-sm opacity-90">Itens Favoritos</div>
                        </div>
                        <div class="text-3xl opacity-80">
                            <i class="fas fa-heart"></i>
                        </div>
                    </div>
                </div>

                <div class="wishlist-card rounded-xl p-6 shadow-lg hover-lift text-white">
                    <div class="flex items-center justify-between">
                        <div>
                            <div class="text-2xl font-bold">Gold</div>
                            <div class="text-sm opacity-90">Status Premium</div>
                        </div>
                        <div class="text-3xl opacity-80">
                            <i class="fas fa-crown"></i>
                        </div>
                    </div>
                </div>

                <!-- Endereços -->
                <div class="profile-card rounded-xl p-6 shadow-lg hover-lift">
                    <h3 class="text-xl font-bold text-gray-900 mb-4">Endereços Salvos</h3>
                    
                    <div class="space-y-3">
                        <div class="p-3 bg-gray-50 rounded-lg">
                            <div class="flex items-center justify-between">
                                <div class="font-semibold text-gray-900">Casa</div>
                                <div class="badge badge-primary">Principal</div>
                            </div>
                            <div class="text-sm text-gray-600 mt-1">
                                Rua das Flores, 123<br>
                                São Paulo - SP, 01234-567
                            </div>
                        </div>
                        
                        <div class="p-3 bg-gray-50 rounded-lg">
                            <div class="font-semibold text-gray-900">Trabalho</div>
                            <div class="text-sm text-gray-600 mt-1">
                                Av. Paulista, 1000<br>
                                São Paulo - SP, 01310-100
                            </div>
                        </div>
                    </div>
                    
                    <button class="btn btn-outline btn-primary w-full mt-4">
                        <i class="fas fa-plus mr-2"></i>
                        Adicionar Endereço
                    </button>
                </div>

                <!-- Cupons -->
                <div class="profile-card rounded-xl p-6 shadow-lg hover-lift">
                    <h3 class="text-xl font-bold text-gray-900 mb-4">Cupons Disponíveis</h3>
                    
                    <div class="space-y-3">
                        <div class="p-3 bg-gradient-to-r from-yellow-400 to-orange-500 rounded-lg text-white">
                            <div class="font-semibold">DESCONTO20</div>
                            <div class="text-sm opacity-90">20% OFF em eletrônicos</div>
                            <div class="text-xs opacity-80">Válido até 31/07/2025</div>
                        </div>
                        
                        <div class="p-3 bg-gradient-to-r from-green-400 to-blue-500 rounded-lg text-white">
                            <div class="font-semibold">FRETE10</div>
                            <div class="text-sm opacity-90">Frete grátis acima de R$ 99</div>
                            <div class="text-xs opacity-80">Válido até 15/08/2025</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </main>

     <div  class="w-full">     
        <img height="400px" :src="profile?.avatar" />
            <h2>{{ profile?.first_name }}</h2>
            <h2>{{ profile?.last_name }}</h2>
            <h2>{{ profile?.full_name }}</h2>
            <p>{{ profile?.username }}</p>
            <p>{{ profile?.nickname }}</p>
            <p>{{ profile?.birth_date }}</p>
            <p>{{ profile?.whatsapp }}</p>
            <p>{{ profile?.phone }}</p>         
            <p>{{ profile?.bio }}</p>
            <h1>{{ $route.params.id }} - {{ route.params.id }}</h1>
    </div>

    </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types';

const route = useRoute();
const config = useRuntimeConfig();
// When accessing /posts/1, route.params.id will be 1
console.log(route.params.id);
const id = route.params.id;



const { pending, data: profile } = await useLazyFetch<Profile>(`${config.public.baseURL}/api/profiles/${id}`);
</script>

<style scoped>
@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css');
 .gradient-bg {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }
        .profile-card {
            backdrop-filter: blur(10px);
            background: rgba(255, 255, 255, 0.95);
        }
        .hover-lift {
            transition: all 0.3s ease;
        }
        .hover-lift:hover {
            transform: translateY(-5px);
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
        }
        .stats-card {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        }
        .orders-card {
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
        }
        .wishlist-card {
            background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
        }
        .animate-float {
            animation: float 3s ease-in-out infinite;
        }
        @keyframes float {
            0%, 100% { transform: translateY(0px); }
            50% { transform: translateY(-10px); }
        }
</style>
