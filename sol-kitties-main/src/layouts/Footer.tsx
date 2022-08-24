import React from 'react'

import { ReactComponent as Discord } from 'assets/icons/discorod.svg'

const Footer = () => {
  return (
    <div className='px-[25px] py-[10px]'>
      <div className='flex items-center justify-center'>
        <Discord className='w-[30px] cursor-pointer transition ease-in-out duration-300 hover:-translate-1 hover:scale-110' />
      </div>
      <p className='text-lg text-center text-[#6C6C6D]'>All rights reserved © Sol Kittes 2022</p>
    </div>
  )
}

export default Footer