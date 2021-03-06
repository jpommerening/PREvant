/*-
 * ========================LICENSE_START=================================
 * PREvant Frontend
 * %%
 * Copyright (C) 2018 - 2019 aixigo AG
 * %%
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * =========================LICENSE_END==================================
 */

<template>
   <dlg ref="dialog" :title="'Shutdown ' + appName" :error-status="errorStatus" :error-status-text="errorStatusText">
      <template slot="body">
         <p>Do you really want to shutdown <b>{{ appName }}</b>? Confirm by typing in the app:</p>

         <div class="form-group">
            <input
                  type="name"
                  class="form-control"
                  placeholder="Enter app name"
                  v-model="confirmedAppName"
                  :disabled="noInteraction ? true : false"
                  @keyup="keyPressed">
         </div>
      </template>
      <template slot="footer">
         <button
               type="button"
               class="btn btn-outline-danger"
               @click="deleteApp()"
               :disabled="noInteraction || confirmedAppName !== appName">
            <font-awesome-icon icon="spinner" spin v-if="noInteraction"/> Confirm
         </button>
      </template>
   </dlg>
</template>

<script>
   import Dialog from './Dialog.vue';

   export default {
      data() {
         return {
            errorStatus: null,
            errorStatusText: null,
            confirmedAppName: '',
            noInteraction: false
         }
      },
      components: {
         'dlg': Dialog
      },
      props: {
         appName: {type: String}
      },
      methods: {
         open() {
            this.$refs.dialog.open();
         },
         keyPressed(e) {
            if (e.keyCode === 13) {
               this.deleteApp();
            }
         },
         deleteApp() {
            if (this.confirmedAppName !== this.appName) {
               return;
            }

            this.errorStatus = null;
            this.errorStatusText = null;
            this.noInteraction = true;

            this.$http.delete(`/api/apps/${this.confirmedAppName}`)
               .then(r => {
                  this.noInteraction = false;
                  this.$refs.dialog.close();

                  // TODO: use a vue.js event to refetch all apps
                  location.reload();
               }, err => {
                  this.noInteraction = false;
                  this.errorStatus = err.status;
                  this.errorStatusText = err.statusText;
               });
         }
      }
   }
</script>
