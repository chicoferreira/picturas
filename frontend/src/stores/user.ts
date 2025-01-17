import { ref, computed } from 'vue'
import { defineStore } from 'pinia'


export const useUserStore = defineStore('counter', () => {
  const name = ref<string>('')
  const email = ref<string>('')
  const token = ref<string>('') // TODO alterar dps para validar se o token é valido
  const prenium = ref(false)
  const uuid = ref<string>('') // TODO não sei se vai ter de se mudar o tipo
  const avatar = ref<string>('') // TODO path da imagem do avatar ou outra coisa


  function logout() {
    token.value = ''
    name.value= ''
    email.value= ''
    prenium.value= false
    uuid.value = ''
  }

  function login(nameInput:string, emailInput:string, tokenInput:string, preniumInput:boolean, uuidInput:string) {
    token.value = tokenInput
    name.value= nameInput
    email.value= emailInput
    prenium.value= preniumInput
    uuid.value = uuidInput
  }

  function loggedIn() {
    // TODO: implementar a validação do token
    return token.value !== '' && name.value !== '' && email.value !== '' && uuid.value !== ''
  }
  return { name,email,token,prenium,uuid, logout, login,loggedIn}
})
