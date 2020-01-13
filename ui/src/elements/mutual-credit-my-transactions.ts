import { LitElement, property, html } from 'lit-element';
import { ApolloClient } from 'apollo-boost';

import { ApolloClientModule } from '@uprtcl/common';
import { moduleConnect } from '@uprtcl/micro-orchestrator';

import '@material/mwc-top-app-bar';
import '@authentic/mwc-list';

import { GET_MY_TRANSACTION } from '../graphql/queries';
import { Transaction } from '../types';

export class MyTransactions extends moduleConnect(LitElement) {
  @property({ type: Object })
  transactions!: Array<Transaction>;

  async firstUpdated() {
    const client: ApolloClient<any> = this.request(
      ApolloClientModule.types.Client
    );
    const result = await client.query({
      query: GET_MY_TRANSACTION
    });

    this.transactions = result.data.myTransactions;
  }

  render() {
    if (!this.transactions)
      return html`
        <span>Loading...</span>
      `;

    return html`
      <mwc-list>
        ${this.transactions.map(
          transaction => html`
            <mwc-list-item>
              ${transaction.sender} => ${transaction.receiver},
              ${transaction.amount}
            </mwc-list-item>
            <mwc-list-divider></mwc-list-divider>
          `
        )}
      </mwc-list>
    `;
  }

}
