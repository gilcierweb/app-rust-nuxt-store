import { defineRule, configure } from 'vee-validate'
import { required, email, numeric, image } from '@vee-validate/rules'
import { localize, setLocale } from '@vee-validate/i18n'
import ptBR from '@vee-validate/i18n/dist/locale/pt_BR.json'

export default defineNuxtPlugin(() => {
  defineRule('required', required)
  defineRule('email', email)
  defineRule('numeric', numeric)
  defineRule('image', image)

  configure({
    generateMessage: localize({ pt_BR: ptBR }),
    validateOnInput: false,
    validateOnBlur: false,
    validateOnModelUpdate: false,
  })

  setLocale('pt_BR')
})