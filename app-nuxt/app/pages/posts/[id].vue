<template>
    <div>
        <div v-if="status === 'pending'">
            {{ t('pages.posts.detail.loading') }} <br />
            <span class="loading loading-spinner size-32"></span>
        </div>

        <!-- Hero Section com Fundo Colorido -->
        <section
            v-else
            class="bg-gradient-to-br from-purple-600 via-pink-600 to-orange-500 relative overflow-hidden"
        >
            <div class="absolute inset-0 bg-black/20"></div>
            <div
                class="absolute inset-0 bg-gradient-to-t from-black/30 via-transparent to-transparent"
            ></div>

            <!-- Elementos Decorativos -->
            <div
                class="absolute top-10 left-10 w-20 h-20 bg-white/10 rounded-full blur-xl"
            ></div>
            <div
                class="absolute bottom-20 right-20 w-32 h-32 bg-white/5 rounded-full blur-2xl"
            ></div>
            <div
                class="absolute top-1/2 left-1/3 w-16 h-16 bg-pink-300/20 rounded-full blur-lg"
            ></div>

            <div
                class="max-w-8xl mx-auto px-4 sm:px-6 lg:px-8 py-20 relative z-10"
            >
                <!-- Breadcrumb -->
                <nav
                    class="flex items-center space-x-2 text-sm text-white/80 mb-8"
                >
                    <a href="#" class="hover:text-white transition-colors"
                        >{{ t('pages.posts.detail.breadcrumb.home') }}</a
                    >
                    <i class="fas fa-chevron-right text-xs"></i>
                    <a href="#" class="hover:text-white transition-colors"
                        >{{ t('pages.posts.detail.breadcrumb.blog') }}</a
                    >
                    <i class="fas fa-chevron-right text-xs"></i>
                    <span class="text-white/60">{{ t('pages.posts.detail.breadcrumb.guides') }}</span>
                </nav>

                <!-- Categoria -->
                <div class="mb-6">
                    <span
                        class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-white/20 text-white backdrop-blur-sm"
                    >
                        <i class="fas fa-tag mr-2"></i>
                        {{ t('pages.posts.detail.category') }}
                    </span>
                </div>

                <!-- Título -->
                <h1
                    class="text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-6 leading-tight"
                >
                    {{ post?.title }}
                </h1>

                <!-- Subtítulo -->
                <p class="text-xl text-white/90 mb-8 max-w-3xl leading-relaxed">
                    Um guia completo com dicas essenciais para fazer a melhor
                    escolha na hora de comprar online, economizando tempo e
                    dinheiro.
                    {{ $route.params.id }} - {{ route.params.id }}
                </p>

                <!-- Meta Informações -->
                <div class="flex flex-wrap items-center gap-6 text-white/80">
                    <div class="flex items-center">
                        <img
                            src="https://images.unsplash.com/photo-1494790108755-2616b612b1a7?w=40&h=40&fit=crop&crop=faces"
                            alt="Autor"
                            class="w-10 h-10 rounded-full mr-3 border-2 border-white/20"
                        />
                        <div>
                            <p class="text-sm font-medium text-white">
                                {{ t('pages.posts.detail.author.name') }}
                            </p>
                            <p class="text-xs text-white/70">
                                {{ t('pages.posts.detail.author.role') }}
                            </p>
                        </div>
                    </div>

                    <div class="flex items-center space-x-4 text-sm">
                        <span class="flex items-center">
                            <i class="fas fa-calendar-alt mr-2"></i>
                            15 Jul 2024
                        </span>
                        <span class="flex items-center">
                            <i class="fas fa-clock mr-2"></i>
                            {{ t('pages.posts.detail.meta.readingTime', { n: 8 }) }}
                        </span>
                        <span class="flex items-center">
                            <i class="fas fa-eye mr-2"></i>
                            {{ t('pages.posts.detail.meta.views', { n: '2.5k' }) }}
                        </span>
                        <span
                            :color="post?.status ? 'green' : 'red'"
                            variant="flat"
                        >
                            {{ post?.status }}
                        </span>
                    </div>
                </div>
            </div>
        </section>

        <!-- Conteúdo Principal -->
        <main class="max-w-8xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <!-- Botões de Compartilhamento Flutuantes -->
            <div
                class="fixed left-6 top-1/2 transform -translate-y-1/2 hidden lg:flex flex-col space-y-3 z-40"
            >
                <button
                    class="w-12 h-12 bg-blue-600 hover:bg-blue-700 text-white rounded-full flex items-center justify-center transition-all duration-300 hover:scale-110 shadow-lg"
                >
                    <i class="fab fa-facebook-f"></i>
                </button>
                <button
                    class="w-12 h-12 bg-sky-500 hover:bg-sky-600 text-white rounded-full flex items-center justify-center transition-all duration-300 hover:scale-110 shadow-lg"
                >
                    <i class="fab fa-twitter"></i>
                </button>
                <button
                    class="w-12 h-12 bg-green-600 hover:bg-green-700 text-white rounded-full flex items-center justify-center transition-all duration-300 hover:scale-110 shadow-lg"
                >
                    <i class="fab fa-whatsapp"></i>
                </button>
                <button
                    class="w-12 h-12 bg-blue-700 hover:bg-blue-800 text-white rounded-full flex items-center justify-center transition-all duration-300 hover:scale-110 shadow-lg"
                >
                    <i class="fab fa-linkedin-in"></i>
                </button>
            </div>

            <div class="grid lg:grid-cols-4 gap-12">
                <!-- Conteúdo do Post -->
                <article class="lg:col-span-3">
                    <!-- Imagem Principal -->
                    <div class="mb-12 rounded-2xl overflow-hidden shadow-2xl">
                        <NuxtImg
                            src="https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=800&h=400&fit=crop"
                            loading="lazy"
                            :alt="post?.title"
                            class="w-full h-96 object-cover"
                        />
                    </div>

                    <!-- Conteúdo -->
                    <div class="prose prose-lg max-w-none">
                        <div class="text-gray-700 mb-6 leading-relaxed">
                            {{ post?.content }}
                        </div>

                        <!-- Introdução -->
                        <div
                            class="bg-gradient-to-r from-purple-50 to-pink-50 border-l-4 border-purple-500 p-6 mb-8 rounded-r-xl"
                        >
                            <p
                                class="text-gray-700 text-lg leading-relaxed m-0"
                            >
                                {{ t('pages.posts.detail.sections.introduction') }}
                            </p>
                        </div>

                        <h2
                            class="text-3xl font-bold text-gray-900 mb-6 flex items-center"
                        >
                            <span
                                class="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm mr-3"
                                >1</span
                            >
                            {{ t('pages.posts.detail.sections.step1') }}
                        </h2>

                        <p class="text-gray-700 mb-6 leading-relaxed">
                            Antes de começar a pesquisar, é fundamental ter
                            clareza sobre o que você realmente precisa. Faça uma
                            lista detalhada das funcionalidades essenciais
                            versus as desejáveis.
                        </p>

                        <!-- Cards de Dicas -->
                        <div class="grid md:grid-cols-2 gap-6 my-10">
                            <div
                                class="bg-green-50 p-6 rounded-xl border border-green-200"
                            >
                                <div class="flex items-center mb-4">
                                    <div
                                        class="w-10 h-10 bg-green-500 rounded-full flex items-center justify-center"
                                    >
                                        <i
                                            class="fas fa-lightbulb text-white"
                                        ></i>
                                    </div>
                                    <h3
                                        class="text-lg font-semibold text-green-800 ml-3"
                                    >
                                        {{ t('pages.posts.detail.sections.tipTitle') }}
                                    </h3>
                                </div>
                                <p class="text-green-700 m-0">
                                    {{ t('pages.posts.detail.sections.tipText') }}
                                </p>
                            </div>

                            <div
                                class="bg-blue-50 p-6 rounded-xl border border-blue-200"
                            >
                                <div class="flex items-center mb-4">
                                    <div
                                        class="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center"
                                    >
                                        <i class="fas fa-star text-white"></i>
                                    </div>
                                    <h3
                                        class="text-lg font-semibold text-blue-800 ml-3"
                                    >
                                        {{ t('pages.posts.detail.sections.reviewTitle') }}
                                    </h3>
                                </div>
                                <p class="text-blue-700 m-0">
                                    {{ t('pages.posts.detail.sections.reviewText') }}
                                </p>
                            </div>
                        </div>

                        <h2
                            class="text-3xl font-bold text-gray-900 mb-6 flex items-center"
                        >
                            <span
                                class="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm mr-3"
                                >2</span
                            >
                            {{ t('pages.posts.detail.sections.step2') }}
                        </h2>

                        <p class="text-gray-700 mb-6 leading-relaxed">
                            A pesquisa é a etapa mais importante do processo de
                            compra. Use diferentes plataformas para comparar
                            preços, características e avaliações dos produtos.
                        </p>

                        <!-- Lista de Verificação -->
                        <div class="bg-gray-50 p-8 rounded-xl my-10">
                            <h3
                                class="text-xl font-semibold text-gray-900 mb-6 flex items-center"
                            >
                                <i
                                    class="fas fa-check-circle text-purple-500 mr-3"
                                ></i>
                                {{ t('pages.posts.detail.checklist.title') }}
                            </h3>
                            <div class="space-y-3">
                                <div class="flex items-center">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-purple-600 rounded mr-3"
                                    />
                                    <span class="text-gray-700"
                                        >{{ t('pages.posts.detail.checklist.items.specs') }}</span
                                    >
                                </div>
                                <div class="flex items-center">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-purple-600 rounded mr-3"
                                    />
                                    <span class="text-gray-700"
                                        >{{ t('pages.posts.detail.checklist.items.reviews') }}</span
                                    >
                                </div>
                                <div class="flex items-center">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-purple-600 rounded mr-3"
                                    />
                                    <span class="text-gray-700"
                                        >{{ t('pages.posts.detail.checklist.items.prices') }}</span
                                    >
                                </div>
                                <div class="flex items-center">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-purple-600 rounded mr-3"
                                    />
                                    <span class="text-gray-700"
                                        >{{ t('pages.posts.detail.checklist.items.returnPolicy') }}</span
                                    >
                                </div>
                                <div class="flex items-center">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-purple-600 rounded mr-3"
                                    />
                                    <span class="text-gray-700"
                                        >{{ t('pages.posts.detail.checklist.items.warranty') }}</span
                                    >
                                </div>
                            </div>
                        </div>

                        <h2
                            class="text-3xl font-bold text-gray-900 mb-6 flex items-center"
                        >
                            <span
                                class="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm mr-3"
                                >3</span
                            >
                            {{ t('pages.posts.detail.sections.step3') }}
                        </h2>

                        <p class="text-gray-700 mb-6 leading-relaxed">
                            Depois de toda a pesquisa, é hora de tomar a
                            decisão. Lembre-se de que a escolha perfeita é
                            aquela que melhor atende às suas necessidades
                            específicas, não necessariamente a mais cara ou a
                            mais popular.
                        </p>

                        <!-- CTA -->
                        <div
                            class="bg-gradient-to-r from-purple-600 to-pink-600 p-8 rounded-2xl text-center my-12"
                        >
                            <h3 class="text-2xl font-bold text-white mb-4">
                                {{ t('pages.posts.detail.cta.title') }}
                            </h3>
                            <p class="text-white/90 mb-6">
                                {{ t('pages.posts.detail.cta.description') }}
                            </p>
                            <button
                                class="bg-white text-purple-600 px-8 py-3 rounded-full font-semibold hover:shadow-lg transition-all duration-300 hover:scale-105"
                            >
                                {{ t('pages.posts.detail.cta.button') }}
                            </button>
                        </div>
                    </div>

                    <!-- Tags -->
                    <div class="flex flex-wrap gap-2 my-8">
                        <span
                            class="px-3 py-1 bg-purple-100 text-purple-700 rounded-full text-sm font-medium"
                            >{{ t('pages.posts.detail.tags.onlineShopping') }}</span
                        >
                        <span
                            class="px-3 py-1 bg-pink-100 text-pink-700 rounded-full text-sm font-medium"
                            >{{ t('pages.posts.detail.tags.guides') }}</span
                        >
                        <span
                            class="px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm font-medium"
                            >{{ t('pages.posts.detail.tags.ecommerce') }}</span
                        >
                        <span
                            class="px-3 py-1 bg-green-100 text-green-700 rounded-full text-sm font-medium"
                            >{{ t('pages.posts.detail.tags.tips') }}</span
                        >
                    </div>

                    <!-- Compartilhamento -->
                    <div class="border-t border-b border-gray-200 py-6 my-8">
                        <div class="flex items-center justify-between">
                            <div>
                                <h3
                                    class="text-lg font-semibold text-gray-900 mb-2"
                                >
                                    {{ t('pages.posts.detail.share.title') }}
                                </h3>
                                <p class="text-gray-600 text-sm">
                                    {{ t('pages.posts.detail.share.description') }}
                                </p>
                            </div>
                            <div class="flex space-x-3">
                                <button
                                    class="flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                                >
                                    <i class="fab fa-facebook-f mr-2"></i>
                                    {{ t('pages.posts.detail.share.facebook') }}
                                </button>
                                <button
                                    class="flex items-center px-4 py-2 bg-sky-500 text-white rounded-lg hover:bg-sky-600 transition-colors"
                                >
                                    <i class="fab fa-twitter mr-2"></i>
                                    {{ t('pages.posts.detail.share.twitter') }}
                                </button>
                            </div>
                        </div>
                    </div>
                </article>

                <!-- Sidebar -->
                <aside class="lg:col-span-1 space-y-8">
                    <!-- Sobre o Autor -->
                    <div class="bg-white p-6 rounded-xl shadow-lg">
                        <h3 class="font-semibold text-gray-900 mb-4">
                            {{ t('pages.posts.detail.sidebar.aboutAuthor') }}
                        </h3>
                        <div class="text-center">
                            <img
                                src="https://images.unsplash.com/photo-1494790108755-2616b612b1a7?w=80&h=80&fit=crop&crop=faces"
                                alt="Maria Silva"
                                class="w-20 h-20 rounded-full mx-auto mb-4"
                            />
                            <h4 class="font-semibold text-gray-900">
                                Maria Silva
                            </h4>
                            <p class="text-sm text-purple-600 mb-3">
                                {{ t('pages.posts.detail.author.role') }}
                            </p>
                            <p class="text-sm text-gray-600 mb-4">
                                {{ t('pages.posts.detail.author.bio') }}
                            </p>
                            <button
                                class="w-full bg-purple-600 text-white py-2 rounded-lg text-sm font-medium hover:bg-purple-700 transition-colors"
                            >
                                {{ t('pages.posts.detail.sidebar.followAuthor') }}
                            </button>
                        </div>
                    </div>

                    <!-- Posts Relacionados -->
                    <div class="bg-white p-6 rounded-xl shadow-lg">
                        <h3 class="font-semibold text-gray-900 mb-4">
                            {{ t('pages.posts.detail.sidebar.relatedPosts') }}
                        </h3>
                        <div class="space-y-4">
                            <a href="#" class="group block">
                                <div class="flex">
                                    <img
                                        src="https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=60&h=60&fit=crop"
                                        alt="Post"
                                        class="w-15 h-15 rounded-lg object-cover mr-3"
                                    />
                                    <div>
                                        <h4
                                            class="text-sm font-medium text-gray-900 group-hover:text-purple-600 transition-colors line-clamp-2"
                                        >
                                            {{ t('pages.posts.detail.relatedPosts.post1') }}
                                        </h4>
                                        <p class="text-xs text-gray-500 mt-1">
                                            {{ t('pages.posts.detail.relatedPosts.time1') }}
                                        </p>
                                    </div>
                                </div>
                            </a>

                            <a href="#" class="group block">
                                <div class="flex">
                                    <img
                                        src="https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=60&h=60&fit=crop"
                                        alt="Post"
                                        class="w-15 h-15 rounded-lg object-cover mr-3"
                                    />
                                    <div>
                                        <h4
                                            class="text-sm font-medium text-gray-900 group-hover:text-purple-600 transition-colors line-clamp-2"
                                        >
                                            {{ t('pages.posts.detail.relatedPosts.post2') }}
                                        </h4>
                                        <p class="text-xs text-gray-500 mt-1">
                                            {{ t('pages.posts.detail.relatedPosts.time2') }}
                                        </p>
                                    </div>
                                </div>
                            </a>

                            <a href="#" class="group block">
                                <div class="flex">
                                    <img
                                        src="https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=60&h=60&fit=crop"
                                        alt="Post"
                                        class="w-15 h-15 rounded-lg object-cover mr-3"
                                    />
                                    <div>
                                        <h4
                                            class="text-sm font-medium text-gray-900 group-hover:text-purple-600 transition-colors line-clamp-2"
                                        >
                                            {{ t('pages.posts.detail.relatedPosts.post3') }}
                                        </h4>
                                        <p class="text-xs text-gray-500 mt-1">
                                            {{ t('pages.posts.detail.relatedPosts.time3') }}
                                        </p>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>

                    <!-- Newsletter -->
                    <div
                        class="bg-gradient-to-br from-purple-600 to-pink-600 p-6 rounded-xl text-white"
                    >
                        <h3 class="font-semibold mb-3">{{ t('pages.posts.detail.sidebar.newsletter') }}</h3>
                        <p class="text-sm text-white/90 mb-4">
                            {{ t('pages.posts.detail.sidebar.newsletterText') }}
                        </p>
                        <div class="space-y-3">
                            <input
                                type="email"
                                :placeholder="t('pages.posts.detail.sidebar.newsletterPlaceholder')"
                                class="w-full px-3 py-2 rounded-lg text-gray-900 placeholder-gray-500 text-sm"
                            />
                            <button
                                class="w-full bg-white text-purple-600 py-2 rounded-lg text-sm font-medium hover:shadow-lg transition-all"
                            >
                                {{ t('pages.posts.detail.sidebar.newsletterButton') }}
                            </button>
                        </div>
                    </div>
                </aside>
            </div>
        </main>

        <!-- Scroll to Top -->
        <button
            id="scrollTop"
            class="fixed bottom-6 right-6 w-12 h-12 bg-purple-600 text-white rounded-full shadow-lg hover:bg-purple-700 transition-all duration-300 hover:scale-110 hidden"
        >
            <i class="fas fa-arrow-up"></i>
        </button>
    </div>
</template>

<script setup lang="ts">
import type { Post } from "~/types";
const { t } = useI18n()

const route = useRoute();
const config = useRuntimeConfig();
// When accessing /posts/1, route.params.id will be 1
console.log(route.params.id);
const id = route.params.id;

const { status, data: post } = await useLazyFetch<Post>(
    `${config.public.baseURL}/api/posts/${id}`,
);

// SEO optimization for blog post
useSeoMeta({
  title: post.value?.title || 'Blog Post',
  ogTitle: post.value?.title || 'Blog Post',
  description: post.value?.content?.substring(0, 160) || 'Read our latest blog post with valuable insights and tips.',
  ogDescription: post.value?.content?.substring(0, 160) || 'Read our latest blog post with valuable insights and tips.',
  ogImage: 'https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=1200&h=630&fit=crop',
  ogUrl: `${config.public.baseURL}/posts/${id}`,
  ogType: 'article',
  articleAuthor: 'Maria Silva',
  articlePublishedTime: '2024-07-15T00:00:00Z',
  articleModifiedTime: '2024-07-15T00:00:00Z',
  twitterCard: 'summary_large_image',
  twitterTitle: post.value?.title || 'Blog Post',
  twitterDescription: post.value?.content?.substring(0, 160) || 'Read our latest blog post with valuable insights and tips.',
  twitterImage: 'https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=1200&h=630&fit=crop',
})

// function updateReadingProgress(): void {
//   const article: HTMLElement | null = document.querySelector('article');
//   if (!article) return;

//   const articleTop: number = article.offsetTop;
//   const articleHeight: number = article.offsetHeight;
//   const windowHeight: number = window.innerHeight;
//   const scrollTop: number = window.scrollY;

//   const progress: number = Math.min(100, Math.max(0,
//     ((scrollTop - articleTop + windowHeight) / articleHeight) * 100
//   ));

//   (this as any).readingProgress = progress;
// }

onMounted(() => {
    // Scroll to top functionality
    const scrollTopBtn: HTMLElement | null =
        document.getElementById("scrollTop");

    if (scrollTopBtn) {
        window.addEventListener("scroll", () => {
            if (window.pageYOffset > 300) {
                scrollTopBtn.classList.remove("hidden");
            } else {
                scrollTopBtn.classList.add("hidden");
            }
        });

        scrollTopBtn.addEventListener("click", () => {
            window.scrollTo({ top: 0, behavior: "smooth" });
        });
    } else {
        console.error("Elemento #scrollTop não encontrado");
    }

    // Smooth animations on scroll
    const observer: IntersectionObserver = new IntersectionObserver(
        (entries: IntersectionObserverEntry[]) => {
            entries.forEach((entry: IntersectionObserverEntry) => {
                if (entry.isIntersecting) {
                    (entry.target as HTMLElement).style.opacity = "1";
                    (entry.target as HTMLElement).style.transform =
                        "translateY(0)";
                }
            });
        },
    );

    // Apply animation to elements
    document.querySelectorAll("article > *").forEach((el: Element) => {
        (el as HTMLElement).style.opacity = "0";
        (el as HTMLElement).style.transform = "translateY(20px)";
        (el as HTMLElement).style.transition =
            "opacity 0.6s ease, transform 0.6s ease";
        observer.observe(el);
    });
});
</script>

<style scoped></style>
