<template>
    <div>        
        <footer class="w-full bg-base-200/60 text-base-content/90 text-center">
            <div class="container mx-auto p-4">
                <p>Copyright &copy; {{ new Date().getFullYear() }} App Nuxt - {{ t('footer.copyright') }} - <a
                        href="https://gilcierweb.com.br" title="GilcierWeb"
                        class="link link-primary link-animated">GilcierWeb</a> - {{ t('footer.createdBy') }}
                    <a href="https://nuxt.com/" title="Nuxt 4" class="link link-primary link-animated">Nuxt 4</a>
                </p>

                <div class="col-12 text-center">
                    <p class="p-small">{{ t('footer.bestViewed') }}
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
    </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
</script>
