import React from 'react'

import Flownav from "../../components/navbar"
import { SideBar } from "../../components/sidebar"
import Libro from "../../components/libro"
const Catalogo = () => {
    return (
        <div>
            <Flownav ></Flownav>

            <div className='grid grid-cols-6 pt-10'>
                <div className='col-span-2'><SideBar></SideBar></div>
                <div className='col-span-4'>
                    <div className='grid grid-cols-4 gap-x-10 gap-y-10'>
                        <Libro
                            url="https://images-na.ssl-images-amazon.com/images/S/compressed.photo.goodreads.com/books/1469392221i/31210898.jpg"
                            titulo="El señor de los anillos"
                            autor="Cassandra Clare"
                            año="2007"
                        />
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                        <Libro></Libro>
                    </div>
                </div>
            </div>


        </div>
    )
}

export default Catalogo