<template>
    <footer class="footer mt-auto bg-gray-900 w-full dark:bg-neutral-950">
        <div class="container mx-auto py-10 px-4 sm:px-6 lg:px-8 lg:pt-20 text-gray-100">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 text-gray-100">

                <div class="flex flex-col">
                    <div class="logo mb-4">
                        <a href="/">
                            <img src="" alt="Rust Loco Nuxt Store">
                        </a>
                    </div>
                    <p class="mb-4">Ecommerce Rust Loco Nuxt Store.</p>
                    <h4 class="social-title text-2xl mb-2 text-white">Siga-nos em:</h4>
                    <ul class="social flex list-inside gap-4">
                        <li><a href=""><i class="icon-[tabler--brand-facebook]"></i></a></li>
                        <li><a href=""><i class="icon-[tabler--brand-instagram]"></i></a></li>
                        <li><a href=""><i class="icon-[tabler--brand-twitter]"></i></a></li>
                        <li><a href=""><i class="icon-[tabler--brand-linkedin]"></i></a></li>
                        <li><a href=""><i class="icon-[tabler--brand-pinterest]"></i></a></li>
                        <li><a href=""><i class="icon-[tabler--brand-youtube]"></i></a></li>
                    </ul>
                </div>

                <nav>
                    <h3 class="text-2xl mb-2 text-white">Links</h3>
                    <ul>
                        <li><a href="" class="link link-primary link-animated">Quem Somos</a></li>
                        <li><a href="" class="link link-primary link-animated">FAQ</a></li>
                        <li><a href="" class="link link-primary link-animated">Como Funciona</a></li>
                        <li><a href="" class="link link-primary link-animated">Planos</a></li>
                        <li><a href="" class="link link-primary link-animated">Privacidade</a></li>
                        <li><a href="" class="link link-primary link-animated">Termos</a></li>
                        <li><a href="" class="link link-primary link-animated">Contatos</a></li>
                    </ul>
                </nav>

                <nav>
                    <h3 class="text-2xl mb-2 text-white">Menu</h3>
                    <ul>

                        <li><a href="" class="link link-primary link-animated">Home</a></li>
                        <li><a href="" class="link link-primary link-animated">Produtos</a></li>
                        <li><a href="" class="link link-primary link-animated">Lojas</a></li>
                        <li><a href="" class="link link-primary link-animated">Anuncie</a></li>
                    </ul>
                </nav>

                <nav>
                    <h3 class="text-2xl mb-2">Inscreva-se</h3>
                    <p class="mb-4">Inscreva-se em nossa newsletter para as últimas atualizações</p>
                    <form action="#" method="get" target="_blank" class="newsletter-form">
                        <div
                            class="flex flex-col items-center gap-2 sm:flex-row sm:gap-3 bg-white rounded-lg p-2 dark:bg-neutral-900">
                            <input name="EMAIL" placeholder="Endereço de email" required="required" type="email"
                                class="input py-3 px-4 block w-full border-transparent rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-transparent dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600">
                            <button
                                class="w-full sm:w-auto whitespace-nowrap p-3 inline-flex justify-center items-center gap-x-2 text-sm font-medium rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none">
                                <i class="lni lni-envelope"></i>
                                Inscreva-se
                            </button>
                        </div>
                    </form>
                    <p class="float-end mt-4"><a href="#">Voltar ao topo</a></p>
                </nav>
            </div>
        </div>
    </footer>

    <footer class="w-full bg-base-200/60 text-base-content/90 text-center">
        <div class="container mx-auto p-4">
            <p>Copyright &copy; {{ new Date().getFullYear() }} App Nuxt - All Rights Reserved - <a
                    href="https://gilcierweb.com.br" title="GilcierWeb"
                    class="link link-primary link-animated">GilcierWeb</a> - Site criado com o
                <a href="https://nuxt.com/" title="Nuxt 3" class="link link-primary link-animated">Nuxt 3</a>
            </p>

            <div class="col-12 text-center">
                <p class="p-small">Site melhor visualizado com navegadores modernos como:
                    <a href="https://browsehappy.com/" title="Mozilla Firefox" target="_blank"
                        class="link link-primary link-animated">Firefox</a>,
                    <a href="https://browsehappy.com/" title="Opera" target="_blank"
                        class="link link-primary link-animated">Opera</a>,
                    <a href="https://browsehappy.com/" title="Google Chrome" target="_blank"
                        class="link link-primary link-animated">Google Chrome</a>,
                    <a href="https://browsehappy.com/" title="Apple Safari" target="_blank"
                        class="link link-primary link-animated">Apple Safari</a>,
                    <a href="https://browsehappy.com/" title="Brave" target="_blank"
                        class="link link-primary link-animated">Brave</a>,
                    <a href="https://browsehappy.com/" title="Vivaldi" target="_blank"
                        class="link link-primary link-animated">Vivaldi</a>
                </p>
            </div>

        </div>
    </footer>

</template>
