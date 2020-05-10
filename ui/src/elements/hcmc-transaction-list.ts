import { LitElement, property, html } from 'lit-element';
import { ApolloClient } from 'apollo-boost';

import { ApolloClientModule } from '@uprtcl/graphql';
import { moduleConnect } from '@uprtcl/micro-orchestrator';

import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';

import { GET_MY_TRANSACTIONS } from '../graphql/queries';
import { Transaction } from '../types';
import { Agent } from 'holochain-profiles';
import { sharedStyles } from './sharedStyles';
import { dateString } from 'src/utils';

export class MCTransactionList extends moduleConnect(LitElement) {
  @property({ type: String })
  myAgentId!: string;

  @property({ type: Object, attribute: false })
  transactions!: Array<Transaction>;

  static get styles() {
    return sharedStyles;
  }

  async firstUpdated() {
    const client: ApolloClient<any> = this.request(
      ApolloClientModule.bindings.Client
    );
    const result = await client.query({
      query: GET_MY_TRANSACTIONS,
      fetchPolicy: 'network-only',
    });

    this.myAgentId = result.data.me.id;
    this.transactions = result.data.me.transactions;
  }

  isOutgoing(transaction: Transaction) {
    return transaction.debtor.id === this.myAgentId;
  }

  getCounterparty(transaction: Transaction): Agent {
    return transaction.creditor.id === this.myAgentId
      ? transaction.debtor
      : transaction.creditor;
  }

  render() {
    return html`<div class="column center-content">
      ${this.renderContent()}
    </div>`;
  }

  renderContent() {
    if (!this.transactions)
      return html`
        <div class="padding center-content">
          <mwc-circular-progress></mwc-circular-progress>
        </div>
      `;

    if (this.transactions.length === 0)
      return html`<div class="padding">
        <span>You have no transactions in your history</span>
      </div>`;

    return html`
      <mwc-list style="width: 100%;">
        ${this.transactions.map(
          (transaction, i) => html`
            <div class="row" style="align-items: center;">
              <mwc-list-item
                twoline
                noninteractive
                graphic="avatar"
                style="flex: 1;"
              >
                <span>
                  ${this.isOutgoing(transaction) ? 'To ' : 'From '}
                  @${this.getCounterparty(transaction).username} on
                  ${dateString(transaction.timestamp)}
                </span>
                <span slot="secondary"
                  >${this.getCounterparty(transaction).id}
                </span>
                <mwc-icon
                  slot="graphic"
                  .style="color: ${this.isOutgoing(transaction)
                    ? 'red'
                    : 'green'}"
                  >${this.isOutgoing(transaction)
                    ? 'call_made'
                    : 'call_received'}</mwc-icon
                >
              </mwc-list-item>

              <span style="font-size: 24px; margin-right: 24px;">
                ${this.isOutgoing(transaction) ? '-' : '+'}${transaction.amount} credits
              </span>
            </div>
            ${i < this.transactions.length - 1
              ? html`<li divider padded role="separator"></li> `
              : html``}
          `
        )}
      </mwc-list>
    `;
  }
}
