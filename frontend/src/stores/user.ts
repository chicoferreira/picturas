import { ref, computed,watch } from 'vue'
import { defineStore } from 'pinia'
import { onMounted } from 'vue'

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
    localStorage.setItem('user', JSON.stringify(toJson()))
  }

  function login(nameInput:string, emailInput:string, tokenInput:string, preniumInput:boolean, uuidInput:string) {
    token.value = tokenInput
    name.value= nameInput
    email.value= emailInput
    prenium.value= preniumInput
    uuid.value = uuidInput
    window.localStorage.setItem('user', JSON.stringify(toJson()))
  }

  function loggedIn() {
    // TODO: implementar a validação do token
    return token.value !== '' && name.value !== '' && email.value !== '' && uuid.value !== ''
  }

  function fromJson(json: any) {
    name.value = json.name
    email.value = json.email
    token.value = json.token
    prenium.value = json.prenium
    uuid.value = json.uuid
  }

  function toJson() {
    return {
      name: name.value,
      email: email.value,
      token: token.value,
      prenium: prenium.value,
      uuid: uuid.value
    }
  }

  onMounted(() => {
    const storageVal = window.localStorage.getItem('user');
    
    if (storageVal) {
      let u = JSON.parse(storageVal);
      fromJson(u);
    }
  })
  

  return { name,email,token,prenium,uuid, logout, login,loggedIn,fromJson,toJson }
})
