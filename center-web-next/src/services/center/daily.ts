import * as center from './center';
import * as piece from './piece';
import * as collection from './collection';


interface MatterContent {
    text: string,
}

function createMatter(id: string, data: MatterContent) {
  return piece.create(id, '', '', data);
}
