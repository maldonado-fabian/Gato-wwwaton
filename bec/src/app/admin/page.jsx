import React from 'react'
import FlowNavAdmin from '../../components/navbardamin'
import { SideBar } from "../../components/sidebar"
import Libro from "../../components/libro"

const Admin = () => {
    return (
        <div>
            <FlowNavAdmin></FlowNavAdmin>
            <div className='grid grid-cols-6 pt-10'>
                <div className='col-span-2'><SideBar></SideBar></div>
                <div className='col-span-4'>
                    <div className='grid grid-cols-4 gap-x-10 gap-y-10'>
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
                        <Libro></Libro>
                    </div>
                </div>
            </div>
        </div>


    )
}


export default Admin