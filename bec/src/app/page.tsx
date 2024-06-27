import React from 'react';
import Flowcarousel from "../components/carousel";
import Flownav from "../components/navbar";
import { SideBar } from "../components/sidebar";
import HorizontalScrollCards from "../components/hscrollcards";
import { Button } from 'flowbite-react';

const Page = () => {
  // Datos de ejemplo para cada categoría
  const librosAñadidosRecientemente = [
    { url: '/ruta/a/imagen1.jpg', titulo: 'Libro 1', autor: 'Autor 1', año: '2023' },
    { url: '/ruta/a/imagen2.jpg', titulo: 'Libro 2', autor: 'Autor 2', año: '2022' },
    { url: '/ruta/a/imagen1.jpg', titulo: 'Libro 1', autor: 'Autor 1', año: '2023' },
    { url: '/ruta/a/imagen2.jpg', titulo: 'Libro 2', autor: 'Autor 2', año: '2022' },
    { url: '/ruta/a/imagen1.jpg', titulo: 'Libro 1', autor: 'Autor 1', año: '2023' },
    { url: '/ruta/a/imagen2.jpg', titulo: 'Libro 2', autor: 'Autor 2', año: '2022' },
    { url: '/ruta/a/imagen1.jpg', titulo: 'Libro 1', autor: 'Autor 1', año: '2023' },
    { url: '/ruta/a/imagen2.jpg', titulo: 'Libro 2', autor: 'Autor 2', año: '2022' },
    // Agrega más libros según sea necesario
  ];

  const bestSellers = [
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    // Agrega más libros según sea necesario
  ];

  const recomendaciones = [
    { url: '/ruta/a/imagen5.jpg', titulo: 'Libro 5', autor: 'Autor 5', año: '2019' },
    { url: '/ruta/a/imagen6.jpg', titulo: 'Libro 6', autor: 'Autor 6', año: '2018' },
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    { url: '/ruta/a/imagen3.jpg', titulo: 'Libro 3', autor: 'Autor 3', año: '2021' },
    { url: '/ruta/a/imagen4.jpg', titulo: 'Libro 4', autor: 'Autor 4', año: '2020' },
    // Agrega más libros según sea necesario
  ];

  return (
    <div className=''>
      <Flownav />
      <Flowcarousel />
      <div className='grid grid-cols-6 gap-2'>
        
        <div className='row-span-4 col-span-2'>
          <h2 className='text-2xl pt-7 pb-2 text-black'>Categorias</h2>
                  
          <SideBar />
        </div>
        
        <div className='col-span-4'>
          <div className='flex justify-between pt-5'>
            <p className='text-2xl text-black font-bold'>Revisa nuestro catalogo</p>
            <Button pill color="success"> Catalogo Completo</Button>
          </div>
        </div>
        
        <div className='col-span-4'>
          <HorizontalScrollCards title="Añadidos recientemente" libros={librosAñadidosRecientemente} />
        </div>
        
        <div className='col-span-4'>
          <HorizontalScrollCards title="Best Sellers" libros={bestSellers} />
        </div>
        
        <div className='col-span-4'>
          <HorizontalScrollCards title="Recomendaciones" libros={recomendaciones} />
        </div>

      </div>
    </div>
  );
};

export default Page;
