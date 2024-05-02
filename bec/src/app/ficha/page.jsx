import React from 'react'
import Flownav from "../../components/navbar"
import Form from "../../components/form"
const page = () => {
    return (
        <div>
            <Flownav></Flownav>
            <div className="flex justify-center items-center h-screen">
                <div className="bg-gray-200 rounded-lg shadow-lg w-[30rem] h-[30rem] pt-10">
                    <Form></Form>
                </div>
            </div>
        </div>
    )
}

export default page