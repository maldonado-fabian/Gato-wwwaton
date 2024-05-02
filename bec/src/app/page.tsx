import React from 'react'
import Flowcarousel from "../components/carousel"
import Flownav from "../components/navbar"
import {SideBar} from "../components/sidebar"
import HorizontalScrollCards from "../components/hscrollcards"
import { Button } from 'flowbite-react'
const page = () => {
  return (
    <div className=''>
      <Flownav></Flownav>
      <Flowcarousel></Flowcarousel>
      <div className='grid grid-cols-6 gap-2'>
        
        <div className='row-span-4 col-span-2'>
          <h2 className='text-2xl pt-7 pb-2 text-black'>Categorias</h2>
                  
          <SideBar></SideBar></div>
        <div className='col-span-4'>
          <div className='flex justify-between pt-5'>
            <p className='text-2xl text-black font-bold'>Revisa nuestro catalogo</p>
            <Button pill color="success"> Catalogo Completo</Button>

          </div>
        </div>
        <div className='col-span-4'><HorizontalScrollCards title="Añadidos recientemente"/></div>
        <div className='col-span-4'><HorizontalScrollCards title="Best Sellers"/></div>
        <div className='col-span-4'><HorizontalScrollCards title="Recomendaciones"/></div>

      </div>
    
    </div>
  )
}

export default page