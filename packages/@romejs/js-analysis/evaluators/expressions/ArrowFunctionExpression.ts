/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FunctionScope, Scope} from '../../scopes';
import {
	AnyNode,
	ArrowFunctionExpression,
	arrowFunctionExpression,
} from '@romejs/js-ast';
import executeFunction from '../../utils/executeFunction';

export default function ArrowFunctionExpression(node: AnyNode, scope: Scope) {
	node = arrowFunctionExpression.assert(node);

	let thisContext;
	const funcScope = scope.findOptional(FunctionScope);
	if (funcScope !== undefined) {
		thisContext = funcScope.meta.thisContext;
	}

	return executeFunction(node, scope, true, thisContext);
}
