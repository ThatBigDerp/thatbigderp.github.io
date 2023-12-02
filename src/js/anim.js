/* ... */
import { getProject, core, types, ImageUrl} from '@theatre/core'
import studio from '@theatre/studio'
/**
 * Theatre.js
 */

studio.initialize()

import projectState from './statesheet.json'

const project = getProject('Universe In Illustratons', {state: projectState}) 

const sheet = project.sheet('The Universe')


// CONSTS
const common = {
    pos: types.compound({
        x: 0,
        y: 0, // you can use just a simple default value\
    }),
    scale: types.number(0.5, {range: [0,1]}),
    opacity: types.number(1, { range: [0, 1] }), // or use a type constructor to customize
    z: 0,
}
const text = {
    color: types.rgba({ r: 255, g: 0, b: 0, a: 1 }),
}








// Observable Universe
const universeobject = sheet.object('TheVisibleUniverse', {
  ...common
})


universeobject.onValuesChange((universeobject) => {
    const vis_universe = document.getElementById('vis-universe')
    vis_universe.style.transform = `translateX(${universeobject.pos.x}%) scale(${universeobject.scale},${universeobject.scale}) translateY(${universeobject.pos.y}%)`
    vis_universe.style.opacity = universeobject.opacity
    vis_universe.style.zIndex = universeobject.z;
});


// Body Sky Background
const skyobj = sheet.object('Sky', {
    background_position: types.stringLiteral(
        'center',
        // Specify labels for the specific values given in the Details Panel and Keyframe Editor
        { center: 'center', top: 'top', bottom: 'bottom', left: 'left', right: 'right' },
      ),
  })
  
skyobj.onValuesChange((skyobj) => {
    console.log(`${skyobj.background_position}`)
    document.body.style.backgroundPosition = `${skyobj.background_position}`;
});

// Observable Universe Hero Text
const herotextobj = sheet.object('Vis Uni Hero Text', {
    ...common,
   ...text,
})
herotextobj.onValuesChange((herotextobj) => {
    const hero_text = document.getElementById('vis-uni-hero-text')
    hero_text.style.transform = `translateX(${herotextobj.pos.x}%) scale(${herotextobj.scale},${herotextobj.scale}) translateY(${herotextobj.pos.y}%)`
    hero_text.style.opacity = herotextobj.opacity
    hero_text.style.color = `${herotextobj.color}`
    hero_text.style.zIndex = herotextobj.z;
});

const vistextobj = sheet.object('Vis Uni Text', {
    ...common,
   ...text,
})
vistextobj.onValuesChange((vistextobj) => {
    const vis_text = document.getElementById('vis-uni-text')
    vis_text.style.transform = `translateX(${vistextobj.pos.x}%) scale(${vistextobj.scale},${vistextobj.scale}) translateY(${vistextobj.pos.y}%)`
    vis_text.style.opacity = vistextobj.opacity
    vis_text.style.color = `${vistextobj.color}`
    vis_text.style.zIndex = vistextobj.z;
});