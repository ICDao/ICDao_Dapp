import { Identity } from '@dfinity/agent'
import { AuthClient } from '@dfinity/auth-client'
import { getIdentityUrl } from './config'

/*
 * A simple wrapper for the official auth client to initialize it and wrap
 * some of the methods in promises
 */
class AuthClientWrapper {
  constructor () {
    this.authClient = AuthClient
    this.ready = false
    return this
  }

  // Create a new auth client and update it's ready state
  async create () {
    console.log('create')
    this.authClient = await AuthClient.create()
    await this.authClient?.isAuthenticated()
    this.ready = true
  }

  async login () {
    console.log('login')
    console.log(getIdentityUrl())
    return new Promise(async (resolve) => {
      await this.authClient?.login({
        identityProvider: getIdentityUrl(),
        onSuccess: async () => {
          resolve(this.authClient?.getIdentity())
        },
      })
    })
  }

  async logout () {
    return this.authClient?.logout({ returnTo: '/' })
  }

  async getIdentity () {
    return this.authClient?.getIdentity()
  }

  async isAuthenticated () {
    return this.authClient?.isAuthenticated()
  }
}

export const authClient = new AuthClientWrapper()
