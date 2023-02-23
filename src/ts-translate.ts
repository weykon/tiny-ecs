const world = {
    component_vecs: [
        ["weykon", "son", undefined],
        [100, 50, undefined],
        ["car", "plane", undefined],
    ],
    entityCount: 3,
}

// 竖着的，列为对应了entity,是的，
// 每个component都用index作为entity的id标识。

// 横着的, 横对应了 
// - NameComponent, 
// - SpeedComponent, 
// - TransportationComponent

// 我现在用ts试试

function newEntity() {
    world.component_vecs.forEach((e: any) => e.push(undefined));
    return ++world.entityCount;
}
class NameComponent {

}
class SpeedComponent { }
class TransportationComponent { }
function addComponent<T extends Componet>(id, comp: T) {
    let component_datas;
    if (comp instanceof NameComponent) {
        component_datas = world.component_vecs[0];
        component_datas[id] = 'unknown'
    } else if (comp instanceof SpeedComponent) {
        component_datas = world.component_vecs[1];
        component_datas[id] = '30'
    } else if (comp instanceof TransportationComponent) {
        component_datas = world.component_vecs[2];
        component_datas[id] = 'bike'
    }
}
type Componet = NameComponent | SpeedComponent | TransportationComponent
function main() {
    let e1 = newEntity();
}
main()

