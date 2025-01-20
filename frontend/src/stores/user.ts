import { ref, computed,watch } from 'vue'
import { defineStore } from 'pinia'
import { onMounted } from 'vue'

export const useUserStore = defineStore('counter', () => {
  const name = ref<string>('')
  const email = ref<string>('')
//   const token = ref<string>('') // TODO alterar dps para validar se o token é valido
  const premium = ref(false)
  const uuid = ref<string>('') // TODO não sei se vai ter de se mudar o tipo
  const avatar = ref<string>('') // TODO path da imagem do avatar ou outra coisa

  const storedUser = localStorage.getItem('user')
  if (storedUser) {
    try {
      fromJson(JSON.parse(storedUser))
    } catch (err) {
      console.error('Failed to parse user data from localStorage:', err)
      localStorage.removeItem('user')
    }
  }

  watch([name, email, premium, uuid, avatar], () => {
    localStorage.setItem('user', JSON.stringify(toJson()))
  }, { deep: true })

  function logout() {
    // token.value = ''
    name.value= ''
    email.value= ''
    premium.value= false
    uuid.value = ''
    localStorage.removeItem('user');

    const storedUser = localStorage.getItem('user')
    if (storedUser) {
      try {
        fromJson(JSON.parse(storedUser))
      } catch (err) {
        console.error('Failed to parse user data from localStorage:', err)
        localStorage.removeItem('user')
      }
    }
  } 

  function login(nameInput:string, emailInput:string/*, tokenInput:string*/, premiumInput:boolean, uuidInput:string) {
    // token.value = tokenInput
    name.value= nameInput
    email.value= emailInput
    premium.value= premiumInput
    uuid.value = uuidInput
    localStorage.setItem('user', JSON.stringify(toJson()))
  }

  function loggedIn() {
    // TODO: implementar a validação do token
    return /*token.value !== '' && */name.value !== '' && email.value !== '' && uuid.value !== ''
  }

  function fromJson(json: any) {
    name.value = json.name
    email.value = json.email
    // token.value = json.token
    premium.value = json.prenium
    uuid.value = json.uuid
  }

  function toJson() {
    return {
      name: name.value,
      email: email.value,
    //   token: token.value,
      premium: premium.value,
      uuid: uuid.value
    }
  }

  return { name, email/*, token*/, premium, uuid, logout, login, loggedIn}
})