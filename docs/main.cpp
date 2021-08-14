//
// Link with IPHlpAPI.lib
// build: cl /EHsc main.cpp /link IPHlpAPI.lib
//
#include <winsock2.h>
#include <iphlpapi.h>
#include <stdio.h>
#include <windows.h>
#pragma comment(lib, "IPHLPAPI.lib")

#define MALLOC(x) HeapAlloc(GetProcessHeap(), 0, (x))
#define FREE(x) HeapFree(GetProcessHeap(), 0, (x))

/* Note: could also use malloc() and free() */

int __cdecl main()
{

    FIXED_INFO *pFixedInfo;
    ULONG ulOutBufLen;
    DWORD dwRetVal;
    IP_ADDR_STRING *pIPAddr;

    pFixedInfo = (FIXED_INFO *)MALLOC(sizeof(FIXED_INFO));
    if (pFixedInfo == NULL)
    {
        printf("Error allocating memory needed to call GetNetworkParams\n");
        return 1;
    }
    ulOutBufLen = sizeof(FIXED_INFO);
    printf("buflen: %d\n", ulOutBufLen);

    // Make an initial call to GetAdaptersInfo to get
    // the necessary size into the ulOutBufLen variable
    if (GetNetworkParams(pFixedInfo, &ulOutBufLen) == ERROR_BUFFER_OVERFLOW)
    {
        printf("buflen after: %d\n", ulOutBufLen);
        FREE(pFixedInfo);
        pFixedInfo = (FIXED_INFO *)MALLOC(ulOutBufLen);
        if (pFixedInfo == NULL)
        {
            printf("Error allocating memory needed to call GetNetworkParams\n");
            return 1;
        }
    }

    if (dwRetVal = GetNetworkParams(pFixedInfo, &ulOutBufLen) == NO_ERROR)
    {

        printf("Host Name: %s\n", pFixedInfo->HostName);
        printf("Domain Name: %s\n", pFixedInfo->DomainName);

        printf("DNS Servers:\n");
        printf("\t%s\n", pFixedInfo->DnsServerList.IpAddress.String);

        pIPAddr = pFixedInfo->DnsServerList.Next;
        while (pIPAddr)
        {
            printf("\t%s\n", pIPAddr->IpAddress.String);
            pIPAddr = pIPAddr->Next;
        }

        printf("Node Type: ");
        switch (pFixedInfo->NodeType)
        {
        case BROADCAST_NODETYPE:
            printf("Broadcast node\n");
            break;
        case PEER_TO_PEER_NODETYPE:
            printf("Peer to Peer node\n");
            break;
        case MIXED_NODETYPE:
            printf("Mixed node\n");
            break;
        case HYBRID_NODETYPE:
            printf("Hybrid node\n");
            break;
        default:
            printf("Unknown node type %0lx\n", pFixedInfo->NodeType);
            break;
        }

        printf("DHCP scope name: %s\n", pFixedInfo->ScopeId);

        if (pFixedInfo->EnableRouting)
            printf("Routing: enabled\n");
        else
            printf("Routing: disabled\n");

        if (pFixedInfo->EnableProxy)
            printf("ARP proxy: enabled\n");
        else
            printf("ARP Proxy: disabled\n");

        if (pFixedInfo->EnableDns)
            printf("DNS: enabled\n");
        else
            printf("DNS: disabled\n");
    }
    else
    {
        printf("GetNetworkParams failed with error: %d\n", dwRetVal);
        return 1;
    }

    if (pFixedInfo)
        FREE(pFixedInfo);

    return 0;
}
