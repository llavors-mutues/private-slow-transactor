import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/graphql';
import { MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
export declare class MutualCreditModule extends MicroModule {
    protected instance: string;
    static id: string;
    dependencies: string[];
    static bindings: {
        MutualCreditProvider: string;
    };
    constructor(instance: string);
    onLoad(container: interfaces.Container): Promise<void>;
    get submodules(): (GraphQlSchemaModule | i18nextModule)[];
}
