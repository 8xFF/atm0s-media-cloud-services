import { find } from 'lodash'
import {
  AlignHorizontalDistributeCenterIcon,
  ArrowLeftRightIcon,
  BookOpenIcon,
  ChartPieIcon,
  CreditCardIcon,
  Settings2Icon,
  UsersIcon,
} from 'lucide-react'
import { useMemo } from 'react'
import { useLocation, useParams } from 'react-router-dom'

export const useMenu = () => {
  const location = useLocation()
  const params = useParams()
  const projectUrl = useMemo(() => `/projects/${params?.id}`, [params?.id])
  const menu = {
    navMain: [
      {
        title: 'Analytics',
        url: projectUrl,
        icon: ChartPieIcon,
      },
      {
        title: 'Sessions',
        url: `${projectUrl}/sessions`,
        icon: ArrowLeftRightIcon,
      },
      {
        title: 'Rooms',
        url: `${projectUrl}/rooms`,
        icon: AlignHorizontalDistributeCenterIcon,
      },
      {
        title: 'Billing',
        url: `${projectUrl}/billing`,
        icon: CreditCardIcon,
      },
      {
        title: 'Settings',
        url: `${projectUrl}/settings`,
        icon: Settings2Icon,
        items: [
          {
            title: 'Project',
            url: `${projectUrl}/settings`,
          },
          {
            title: 'Members',
            url: `${projectUrl}/members`,
          },
        ],
      },
    ],
    navSecondary: [
      {
        title: 'Documentation',
        url: '/',
        icon: BookOpenIcon,
      },
      {
        title: 'Discord',
        url: 'https://discord.gg/g5KYHRKS52',
        icon: UsersIcon,
      },
    ],
  }

  const isActive = useMemo(() => {
    return find(menu.navMain, (item) =>
      item.url !== '/' ? location.pathname.includes(item.url) : item.url === location.pathname
    )
  }, [location.pathname, menu.navMain])

  return {
    menu,
    isActive,
  }
}
