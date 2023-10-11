<template>
    <div>
        <h4>Register user</h4>
        <input placeholder="username" v-model="username_reg"/>
        <input placeholder="password" type="password" v-model="password_reg"/>
        <button @click="register_user">Register user</button>
        <p>{{ response_register }}</p>
    </div>
    <div>
        <h4>Log in</h4>
        <input placeholder="username" v-model="username_auth"/>
        <input placeholder="password" type="password" v-model="password_auth"/>
        <button @click="login">Log in</button>
        <p>{{ response_auth }}</p>
    </div>
</template>
<script setup lang="js">
    import { ref } from 'vue'; 
    import axios from 'axios';
    import PBKDF2 from 'crypto-js/pbkdf2';

    const response_register = ref("");
    const response_auth = ref("");
    const username_reg = ref("");
    const password_reg = ref("");

    const username_auth = ref("");
    const password_auth = ref("");

    const keySize = 512 / 32;
    const iterations = 1000;

    async function register_user() {
        const data = { username: username_reg.value, password: hash_pass(username_reg.value, password_reg.value) };
        console.log(data.password)
        const result = await axios.post('http://localhost:7878/user', data);
        response_register.value = result.data;
    }

    function hash_pass(username, pass) {
        return PBKDF2(pass, username, { keySize, iterations }).toString();
    }

    async function login() {
        const data = { username: username_auth.value, password: hash_pass(username_auth.value, password_auth.value) };
        const result = await axios.post('http://localhost:7878/auth', data);
        response_auth.value = result.data;
    }
</script>


<style scoped>
div {
    border: solid black;
    margin: 10px;
    padding: 10px;
    display: flex;
    flex-direction: column;
}
</style>