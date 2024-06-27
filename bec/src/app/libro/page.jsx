import React from 'react'
import Flownav from "../../components/navbar"
import Libro from "../../components/libro"

const page = () => {
    return (
        <div>
            <Flownav></Flownav>
            <div className="flex justify-center items-center h-screen">
                <Libro></Libro>
            </div>
        </div>
    )
}

export default page