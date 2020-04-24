import { LitElement, html, property } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import '@authentic/mwc-circular-progress';
import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { ApolloClientModule } from '@uprtcl/graphql';

import { GET_MY_BALANCE } from '../graphql/queries';

export class MyBalance extends moduleConnect(LitElement) {
  client!: ApolloClient<any>;

  @property({ attribute: false, type: Number })
  balance!: number;

  async firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);

    const result = await this.client.query({
      query: GET_MY_BALANCE,
    });

    this.balance = result.data.myBalance;
  }

  render() {
    if (this.balance === undefined)
      return html`<mwc-circular-progress></mwc-circular-progress>`;

    return html` <span>${this.balance}</span> `;
  }
}
