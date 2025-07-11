<template>
    <div class="container mx-auto p-4">
        <footer class="text-center text-sm text-gray-500">

            <nav class="flex items-center justify-center mt-4">
                <ul>
                    <li><NuxtLink to="/privacy" class="link link-primary link-animated">Privacy Policy</NuxtLink></li>
                    <li><NuxtLink to="/terms" class="link link-primary link-animated">Terms of Service</NuxtLink></li>
                </ul>
            </nav>

            <p>Copyright &copy; {{ new Date().getFullYear() }} App Nuxt - All Rights Reserved - <a href="https://gilcierweb.com.br" title="GilcierWeb" class="link link-primary link-animated">GilcierWeb</a> - Site criado com o
                  <a href="https://nuxt.com/" title="Nuxt 3" class="link link-primary link-animated">Nuxt 3</a>
            </p>
                     
            <div class="col-12 text-center">
                <p class="p-small">Site melhor visualizado com navegadores modernos como:
                  <a href="https://browsehappy.com/" title="Mozilla Firefox" target="_blank" class="link link-primary link-animated">Firefox</a>,
                  <a href="https://browsehappy.com/" title="Opera" target="_blank" class="link link-primary link-animated">Opera</a>,
                  <a href="https://browsehappy.com/" title="Google Chrome" target="_blank" class="link link-primary link-animated">Google Chrome</a>,
                  <a href="https://browsehappy.com/" title="Apple Safari" target="_blank" class="link link-primary link-animated">Apple Safari</a>,
                  <a href="https://browsehappy.com/" title="Brave" target="_blank" class="link link-primary link-animated">Brave</a>,
                  <a href="https://browsehappy.com/" title="Vivaldi" target="_blank" class="link link-primary link-animated">Vivaldi</a>
                </p>
              </div>
        </footer>
    </div>
</template>
