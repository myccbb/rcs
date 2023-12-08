export default function Layout({
    children,
}: {
    children: React.ReactNode
}) {
    return (
        <>
            <p>hello</p>
            <section>{children}</section>
            <p>hello end</p>
        </>
    )
}
