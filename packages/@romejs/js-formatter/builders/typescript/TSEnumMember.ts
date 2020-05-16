/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumMember} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';

export default function TSEnumMember(
	builder: Builder,
	node: TSEnumMember,
): Token {
	const tokens: Array<Token> = [builder.tokenize(node.id, node)];

	if (node.initializer) {
		tokens.push(space, '=', space, builder.tokenize(node.initializer, node));
	}

	tokens.push(',');

	return concat(tokens);
}
