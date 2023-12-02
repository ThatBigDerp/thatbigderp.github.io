/* ... */
import { getProject, core, types, ImageUrl} from '@theatre/core'
import studio from '@theatre/studio'

/**
 * Theatre.js
 */

studio.initialize()

const project = getProject('Universe In Illustratons', {
    assets: {
      baseUrl: '../img/assets/',
    },
  }) 

const sheet = project.sheet('The Universe')


// CONSTS
const common = {
    pos: types.compound({
        x: 0,
        y: 0, // you can use just a simple default value\
    }),
    scale: types.number(0.5, {range: [0,1]}),
    opacity: types.number(1, { range: [0, 1] }), // or use a type constructor to customize
}









// Visible Universe
const universeobject = sheet.object('TheVisibleUniverse', {
  ...common
})


universeobject.onValuesChange((universeobject) => {
    const vis_universe = document.getElementById('vis-universe')
    console.log(universeobject.pos.x)
    vis_universe.style.transform = `translateX(${universeobject.pos.x}%) scale(${universeobject.scale},${universeobject.scale}) translateY(${universeobject.pos.y}%)`
    vis_universe.style.opacity = universeobject.opacity
});


// Body
// const skyobj = sheet.object('Sky', {
//     background_position: types.stringLiteral(
//         'center',
//         // Specify labels for the specific values given in the Details Panel and Keyframe Editor
//         { center: 'center', top: 'top', bottom: 'bottom', left: 'left', right: 'right' },
//       ),
//   })
  
// skyobj.onValuesChange((skyobj) => {
//     console.log(`${skyobj.background_position}`)
//     document.body.style.backgroundPosition = `${skyobj.background_position}`;
// });

// Header
// const herotextobj = sheet.object('Hero Text', {
//     common,
//     color: types.rgba({ r: 255, g: 0, b: 0, a: 1 }),
// })
// herotextobj.onValuesChange((herotextobj) => {
//     const hero_text = document.getElementById('vis-uni-hero-text')
//     hero_text.style.transform = `translateX(${herotextobj.common.position.x}%) scale(${herotextobj.common.scale/100},${herotextobj.common.scale/100}) translateY(${herotextobj.common.position.y}%)`
//     hero_text.style.opacity = herotextobj.common.opacity
//     //hero_text.style.scale = objherotextobj.scale/100
//     hero_text.style.color = `${herotextobj.color}`
// });