import { defineRule, configure } from 'vee-validate'
import { required, email, numeric, image, min_value } from '@vee-validate/rules'
import { localize, setLocale } from '@vee-validate/i18n'
import ptBR from '@vee-validate/i18n/dist/locale/pt_BR.json'

export default defineNuxtPlugin(() => {
  defineRule('required', required)
  defineRule('email', email)
  defineRule('numeric', numeric)
  defineRule('image', image)
  defineRule('min_value', min_value)

  configure({
    generateMessage: localize({ pt_BR: ptBR }),
    validateOnInput: false,
    validateOnBlur: false,
    validateOnModelUpdate: false,
  })

  setLocale('pt_BR')
})